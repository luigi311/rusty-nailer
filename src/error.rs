use thiserror::Error;

/// A unified error type.
#[derive(Error, Debug)]
pub enum RustyNailerError {
    /// Wraps errors originating from the `image` crate.
    #[error("Image crate error: {0}")]
    Image(#[from] image::ImageError),

    /// Wraps errors originating from the `fast_image_resize` crate.
    #[error("Fast image resize error: {0}")]
    FastResize(#[from] fast_image_resize::ResizeError),

    /// Wraps standard I/O errors.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),    
}
