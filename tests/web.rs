//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;

use avgcol::*;

wasm_bindgen_test_configure!(run_in_browser);

const DEMO_IMAGE: &[u8] = include_bytes!("image.jpg");

#[wasm_bindgen_test]
fn test_image_average_color() {
    let image = AverageColor::from_bytes(DEMO_IMAGE);

    let image = match image {
        Ok(v) => v,
        Err(_) => panic!("Failed to load image"),
    };

    let expected = AverageColor(178, 180, 172);

    assert_eq!(image, expected);
}
