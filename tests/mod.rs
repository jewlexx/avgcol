//! Test suite for the Web and headless browsers.
use std::assert_eq;

use avgcol::*;

const DEMO_IMAGE: &[u8] = include_bytes!("image.jpg");

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
