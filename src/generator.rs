use std::fmt;
use std::time::Instant;

use image::{DynamicImage, Rgba};
use imageproc::rect::Rect;

pub struct Generator {}

impl Default for Generator {
    fn default() -> Self {
        Generator {}
    }
}

impl Generator {
    pub fn draw(&self, width: u32, height: u32) -> Result<DynamicImage, GeneratorError> {
        let now = Instant::now();
        let imgbuf = imageproc::drawing::draw_filled_rect(
            &image::DynamicImage::new_rgba16(width, height),
            Rect::at(0, 0).of_size(width, height),
            Rgba([50u8, 50u8, 50u8, 50u8]),
        );

        let x = image::DynamicImage::ImageRgba8(imgbuf);
        println!("Rendered in {}ms", now.elapsed().as_millis());

        Ok(x)
    }
}

#[derive(Debug)]
pub enum GeneratorError {
    RenderingError,
}

impl std::error::Error for GeneratorError {}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeneratorError::RenderingError => write!(f, "Rendering Error"),
        }
    }
}
