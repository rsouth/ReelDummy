use std::fmt;
use std::io::Cursor;

use image::{DynamicImage, ImageError};
use serde::Deserialize;

use crate::Drawer;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_picsum_url")]
    picsum_url: String,

    #[serde(default = "default_retry_attempts")]
    retry_attempts: u32,
}

fn default_picsum_url() -> String {
    "https://picsum.photos/".to_string()
}

// TODO
fn default_retry_attempts() -> u32 {
    1
}

pub struct LoremPicsum {
    config: Config,
}

impl Default for LoremPicsum {
    fn default() -> Self {
        let cfg = match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("{:#?}", error),
        };

        LoremPicsum { config: cfg }
    }
}

#[derive(Debug)]
pub enum PicsumError {
    HttpError,
    ImageError,
}

impl std::error::Error for PicsumError {}

impl fmt::Display for PicsumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PicsumError::HttpError => write!(f, "HTTP Error"),
            PicsumError::ImageError => write!(f, "Image Error"),
        }
    }
}

impl From<reqwest::Error> for PicsumError {
    fn from(_: reqwest::Error) -> Self {
        PicsumError::HttpError
    }
}

impl From<image::ImageError> for PicsumError {
    fn from(_: ImageError) -> Self {
        PicsumError::ImageError
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Drawer for LoremPicsum {
    fn draw(&mut self, width: u32, height: u32) -> Result<DynamicImage> {
        println!("Using configuration {:?}", self.config);

        let url = format!("{}/{}/{}", self.config.picsum_url, width, height);
        match reqwest::blocking::get(url)?.bytes() {
            Ok(b) => {
                println!("Downloaded image");
                let reader = image::io::Reader::new(Cursor::new(b.to_vec()))
                    .with_guessed_format()
                    .unwrap();
                Ok(reader.decode()?)
            }
            Err(_) => Err(Box::new(PicsumError::HttpError)),
        }
    }
}
