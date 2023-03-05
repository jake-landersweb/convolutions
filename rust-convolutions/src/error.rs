//! Main Crate Error

use image::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // generic errors
    #[error("Generic {0}")]
    Generic(String),

    // for io errors
    #[error(transparent)]
    IO(#[from] std::io::Error),

    // image errors
    #[error(transparent)]
    ImageError(#[from] image::error::ImageError),
}
