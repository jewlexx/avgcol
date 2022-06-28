mod utils;

use wasm_bindgen::prelude::*;

use image::RgbImage;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! sum {
    ($vec:expr, $sum_type:tt) => {{
        let mut sum: $sum_type = 0 as $sum_type;

        for i in $vec {
            sum += i as $sum_type;
        }

        sum
    }};
}

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AverageColor(pub u64, pub u64, pub u64);

impl Drop for AverageColor {
    fn drop(&mut self) {
        self.0 = 0;
        self.1 = 0;
        self.2 = 0;
    }
}

#[wasm_bindgen]
impl AverageColor {
    fn from_mem<'a>(bytes: impl Into<&'a [u8]>) -> Result<RgbImage, JsError> {
        let bytes: &[u8] = bytes.into();
        let image_data = image::load_from_memory(bytes)?;

        Ok(image_data.to_rgb8())
    }

    fn get_average_color(image_data: RgbImage) -> Result<AverageColor, JsError> {
        let pixels: Vec<_> = image_data.pixels().cloned().collect();
        let pixels_length = pixels.len() as u64;

        let red = sum!(pixels.iter().map(|pixel| pixel[0]), u64) / pixels_length;
        let green = sum!(pixels.iter().map(|pixel| pixel[1]), u64) / pixels_length;
        let blue = sum!(pixels.iter().map(|pixel| pixel[2]), u64) / pixels_length;

        Ok(AverageColor(red, green, blue))
    }

    pub fn from_bytes(image_bytes: &[u8]) -> Result<AverageColor, JsError> {
        let image_data = Self::from_mem(image_bytes)?;

        Self::get_average_color(image_data)
    }

    #[cfg(feature = "remote_image")]
    pub async fn from_url(url: String) -> Result<AverageColor, JsError> {
        let image = reqwest::get(url).await?;
        let bytes: &[u8] = &image.bytes().await?;

        let image_data = Self::from_mem(bytes)?;

        Self::get_average_color(image_data)
    }

    pub fn from_base64(base64: String) -> Result<AverageColor, JsError> {
        let image_bytes = base64::decode(base64)?;
        let image_data = Self::from_mem(image_bytes.as_slice())?;

        Self::get_average_color(image_data)
    }
}
