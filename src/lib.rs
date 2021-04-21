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
pub enum ReelError {
    Error,
}

impl std::error::Error for ReelError {}

impl fmt::Display for ReelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReelError::Error => write!(f, "Reel Error"),
        }
    }
}

impl From<ImageError> for ReelError {
    fn from(_: ImageError) -> Self {
        ReelError::Error
    }
}

impl From<PicsumError> for ReelError {
    fn from(_: PicsumError) -> Self {
        ReelError::Error
    }
}

impl From<GeneratorError> for ReelError {
    fn from(_: GeneratorError) -> Self {
        ReelError::Error
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

impl ReelDummy {
    pub fn with_size(&mut self, width: u32, height: u32) -> &mut ReelDummy {
        self.width = width;
        self.height = height;
        self
    }

    pub fn fetch(&self) -> Result<DynamicImage, ReelError> {
        match self.image_type {
            ImageType::Generated => Generator::default()
                .draw(self.width, self.height)
                .map_err(|_| ReelError::Error),
            ImageType::LoremPicsum => LoremPicsum::default()
                .download(self.width, self.height)
                .map_err(|_| ReelError::Error),
        }
    }
}
