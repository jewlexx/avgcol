use image::RgbImage;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Base64 error: {0}")]
    Base64(#[from] base64::DecodeError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AverageColor(pub u64, pub u64, pub u64);

impl Drop for AverageColor {
    fn drop(&mut self) {
        self.0 = 0;
        self.1 = 0;
        self.2 = 0;
    }
}

impl AverageColor {
    fn from_mem<'a>(bytes: impl Into<&'a [u8]>) -> Result<RgbImage> {
        let bytes: &[u8] = bytes.into();
        let image_data = image::load_from_memory(bytes)?;

        Ok(image_data.to_rgb8())
    }

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

    pub fn from_bytes(image_bytes: &[u8]) -> Result<AverageColor> {
        let image_data = Self::from_mem(image_bytes)?;

        Self::get_average_color(image_data)
    }

    #[cfg(feature = "remote_image")]
    pub async fn from_url(url: String) -> Result<AverageColor> {
        let image = reqwest::get(url).await?;
        let bytes: &[u8] = &image.bytes().await?;

        let image_data = Self::from_mem(bytes)?;

        Self::get_average_color(image_data)
    }

    pub fn from_base64(base64: String) -> Result<AverageColor> {
        let image_bytes = base64::decode(base64)?;
        let image_data = Self::from_mem(image_bytes.as_slice())?;

        Self::get_average_color(image_data)
    }

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

    #[test]
    fn test_image_average_color() {
        let image = AverageColor::from_bytes(DEMO_IMAGE);

        let image = match image {
            Ok(v) => v,
            Err(_) => panic!("Failed to load image"),
        };

        let expected = AverageColor(178, 180, 172);

        assert_eq!(image, expected);
    }
}
