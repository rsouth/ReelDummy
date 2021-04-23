extern crate reqwest;

use std::fmt;

use image::{DynamicImage, ImageError};

use crate::generator::{Generator, GeneratorError};
use crate::lorem_picsum::{LoremPicsum, PicsumError};

mod generator;
mod lorem_picsum;

pub enum ImageType {
    Generated,
    LoremPicsum,
}

#[derive(Debug)]
pub struct ReelError;

impl std::error::Error for ReelError {}

impl fmt::Display for ReelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &_ => {
                write!(f, "Reel Error")
            }
        }
    }
}

impl From<ImageError> for ReelError {
    fn from(_: ImageError) -> Self {
        ReelError {}
    }
}

impl From<PicsumError> for ReelError {
    fn from(_: PicsumError) -> Self {
        ReelError {}
    }
}

impl From<GeneratorError> for ReelError {
    fn from(_: GeneratorError) -> Self {
        ReelError {}
    }
}

pub struct ReelDummy {
    image_type: ImageType,
    height: u32,
    width: u32,
}

impl ReelDummy {
    pub fn new(image_type: ImageType) -> ReelDummy {
        ReelDummy {
            image_type,
            height: 0,
            width: 0,
        }
    }
}

impl Default for ReelDummy {
    fn default() -> Self {
        ReelDummy {
            image_type: ImageType::Generated,
            height: 0,
            width: 0,
        }
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait Drawer {
    fn draw(&mut self, width: u32, height: u32) -> Result<DynamicImage>;
}

impl ReelDummy {
    pub fn with_size(&mut self, width: u32, height: u32) -> &mut ReelDummy {
        self.width = width;
        self.height = height;
        self
    }

    pub fn fetch(&self) -> Result<DynamicImage> {
        match self.image_type {
            ImageType::Generated => Generator::default()
                .draw(self.width, self.height)
                .map_err(|_| ReelError {}.into()),
            ImageType::LoremPicsum => LoremPicsum::default()
                .draw(self.width, self.height)
                .map_err(|_| ReelError {}.into()),
        }
    }
}
