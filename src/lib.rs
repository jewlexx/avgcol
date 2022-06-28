mod utils;

use wasm_bindgen::prelude::*;

use image::{Rgb, RgbImage};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! sum {
    ($vec:expr, $sum_type:tt) => {{
        let mut sum: $sum_type = 0.into();

        for i in $vec {
            sum += i as $sum_type;
        }

        sum
    }};
}

#[wasm_bindgen]
pub struct AverageColor(Rgb<f64>);

#[wasm_bindgen]
impl AverageColor {
    fn from_mem<'a>(bytes: impl Into<&'a [u8]>) -> Result<RgbImage, JsError> {
        let bytes: &[u8] = bytes.into();
        let image_data = image::load_from_memory(bytes)?;

        Ok(image_data.to_rgb8())
    }

    fn get_average_color(image_data: RgbImage) -> Result<AverageColor, JsError> {
        let pixels: Vec<Rgb<u8>> = image_data.pixels().cloned().collect();
        let pixels_length = pixels.len() as f64;

        let red = sum!(pixels.iter().map(|pixel| pixel[0]), f64) / pixels_length;
        let green = sum!(pixels.iter().map(|pixel| pixel[1]), f64) / pixels_length;
        let blue = sum!(pixels.iter().map(|pixel| pixel[2]), f64) / pixels_length;

        let average: [f64; 3] = [red, green, blue];

        Ok(AverageColor(Rgb(average)))
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
