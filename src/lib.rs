#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use image::RgbImage;

#[derive(Debug, thiserror::Error)]
/// Error types for AvgCol
pub enum Error {
    #[error("Image error: {0}")]
    /// Error with parsing images
    Image(#[from] image::ImageError),

    #[error("Failed to decode base64: {0}")]
    /// Error decoding base64
    Base64(#[from] base64::DecodeError),

    #[cfg(feature = "remote_image")]
    #[error("Failed to get remote image: {0}")]
    /// Error interacting with remote images
    Reqwest(#[from] reqwest::Error),
}

/// Result type for AvgCol. Simply wraps the given type with the crate error
/// using the [`std::result::Result`] type
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
/// AverageColor struct containing the rgb values
pub struct AverageColor(pub u64, pub u64, pub u64);

impl Drop for AverageColor {
    fn drop(&mut self) {
        self.0 = 0;
        self.1 = 0;
        self.2 = 0;
    }
}

impl AverageColor {
    fn get_average_color(image_data: RgbImage) -> Result<AverageColor> {
        let pixels: Vec<_> = image_data.pixels().cloned().collect();
        let pixels_length = pixels.len() as u64;

        let red: u64 = pixels.iter().map(|pixel| u64::from(pixel[0])).sum();
        let green: u64 = pixels.iter().map(|pixel| u64::from(pixel[1])).sum();
        let blue: u64 = pixels.iter().map(|pixel| u64::from(pixel[2])).sum();

        Ok(AverageColor(
            red / pixels_length,
            green / pixels_length,
            blue / pixels_length,
        ))
    }

    /// Generate average colour from the given bytes
    pub fn from_bytes<'a>(image_bytes: impl Into<bytes::Bytes>) -> Result<AverageColor> {
        let bytes: bytes::Bytes = image_bytes.into();
        let image_data = image::load_from_memory(&bytes)?.to_rgb8();

        Self::get_average_color(image_data)
    }

    #[cfg(feature = "remote_image")]
    /// Generate average colour from the given image url
    pub async fn from_url(url: impl AsRef<str>) -> Result<AverageColor> {
        let image = reqwest::get(url.as_ref()).await?;
        let bytes = image.bytes().await?;

        Self::from_bytes(bytes)
    }

    /// Generate average colour from the given base64 string
    pub fn from_base64(base64: impl AsRef<str>) -> Result<AverageColor> {
        let image_bytes = base64::decode(base64.as_ref())?;

        Self::from_bytes(image_bytes)
    }

    /// Detect whether the average colour is light or dark
    pub fn is_light(&self) -> bool {
        let red = self.0 as f64;
        let green = self.1 as f64;
        let blue = self.2 as f64;

        let brightness = (red * 0.2126 + green * 0.7152 + blue * 0.0722) / 255.0;

        brightness > 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::AverageColor;

    const DEMO_IMAGE: &[u8] = include_bytes!("../tests/image.jpg");
    const EXPECTED: AverageColor = AverageColor(178, 180, 172);

    #[test]
    fn test_image_average_color() {
        let image = AverageColor::from_bytes(DEMO_IMAGE).expect("Failed to load image");

        assert_eq!(image, EXPECTED);
    }

    #[test]
    fn test_base64_image() {
        let base64_string = base64::encode(DEMO_IMAGE);

        let image = AverageColor::from_base64(base64_string).expect("Failed to load image");

        assert_eq!(image, EXPECTED);
    }

    #[cfg(feature = "remote_image")]
    #[test]
    fn test_remote_image() {
        const REMOTE_DEMO_IMAGE: &str =
            "https://github.com/jewlexx/avgcol/blob/trunk/tests/image.jpg?raw=true";
        use tokio_test::block_on;

        let image = block_on(async { AverageColor::from_url(REMOTE_DEMO_IMAGE).await })
            .expect("Failed to load image");

        assert_eq!(image, EXPECTED);
    }

    #[test]
    fn test_is_light() {
        let image = AverageColor::from_bytes(DEMO_IMAGE).expect("Failed to load image");

        assert!(image.is_light());
    }
}
