use std::fmt;
use std::time::Instant;

use crate::Drawer;
use image::{DynamicImage, Rgba};
use imageproc::rect::Rect;

use imageproc::definitions::Image;
use rusttype::{Font, Point, PositionedGlyph, Scale, VMetrics};

pub struct Generator {
    _font: Font<'static>,
}

impl Default for Generator {
    fn default() -> Self {
        let font_data: &[u8] = include_bytes!("Roboto-Black.ttf");
        let font: Font<'static> = Font::try_from_bytes(font_data).expect("Loading font failed");

        Generator { _font: font }
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Drawer for Generator {
    fn draw(&mut self, width: u32, height: u32) -> Result<DynamicImage> {
        let now = Instant::now();

        let mut imgbuf = imageproc::drawing::draw_filled_rect(
            &image::DynamicImage::new_rgba16(width, height),
            Rect::at(0, 0).of_size(width, height),
            Rgba([50u8, 50u8, 50u8, 50u8]),
        );

        let imgbuf = self.draw_text(&mut imgbuf, width, height);

        println!("Rendered in {}ms", now.elapsed().as_millis());
        Ok(imgbuf)
    }
}

impl Generator {
    fn draw_text(&mut self, imgbuf: &mut Image<Rgba<u8>>, width: u32, height: u32) -> DynamicImage {
        let colour = (75, 75, 75);
        let scale = Scale { x: 50.0, y: 50.0 };

        let text = format!("{} × {}", &width, &height);

        // layout the glyphs in a line with 20 pixels padding
        let v_metrics = self._font.v_metrics(scale);
        let glyphs = self.get_glyph(scale, &text, &v_metrics, 10.0, 10.0);

        // work out the layout size
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        let glyphs_height = (v_metrics.ascent + v_metrics.descent).ceil() as u32;
        let pos_x = (width as f32 / 2.0) - (glyphs_width as f32 / 2.0);
        let pos_y = (height as f32 / 2.0) - (glyphs_height as f32 / 2.0);
        let glyphs = self.get_glyph(scale, &text, &v_metrics, pos_x, pos_y);

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    imgbuf.put_pixel(
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                    )
                });
            }
        }

        image::DynamicImage::ImageRgba8(imgbuf.clone())
    }

    fn get_glyph(
        &mut self,
        scale: Scale,
        text: &String,
        v_metrics: &VMetrics,
        _x: f32,
        _y: f32,
    ) -> Vec<PositionedGlyph> {
        let glyphs: Vec<_> = self
            ._font
            .layout(
                &text,
                scale,
                Point {
                    x: _x,
                    y: _y + v_metrics.ascent,
                },
            )
            .collect();
        glyphs
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
