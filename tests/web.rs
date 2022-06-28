//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use avgcol::*;

wasm_bindgen_test_configure!(run_in_browser);

const DEMO_IMAGE: &[u8] = include_bytes!("image.jpg");

#[wasm_bindgen_test]
fn check_image() {
    let image = AverageColor::from_bytes(DEMO_IMAGE);

    let image = match image {
        Ok(v) => v,
        Err(_) => panic!("Failed to load image"),
    };
}
