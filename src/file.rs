use std::fs::File;
use jxl_oxide::integration::JxlDecoder;
use std::path::Path;
use image::{DynamicImage, ImageReader, Limits};

use crate::error::RustyNailerError;


/// Parses the input file and returns a `DynamicImage`.
pub fn parse_file(input: &str) -> Result<DynamicImage, RustyNailerError> {
    let path = Path::new(input);

    // Check if file exists.
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File {input} not found")
        ).into());
    }

    // Determine the file extension to decide how to parse.
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let dyn_img = match ext.as_str() {
        "jxl" => {
            let file = File::open(input)?;
            let decoder = JxlDecoder::new(file)
                .map_err(|e| image::ImageError::Decoding(
                    image::error::DecodingError::new(
                        image::error::ImageFormatHint::PathExtension("jxl".into()),
                        e
                    )
                ))?;
            image::DynamicImage::from_decoder(decoder)?
        }
        _ => {
            let mut reader = ImageReader::open(input)?;

            // Set the memory limit to 1GB
            let mut limits = Limits::default();
            limits.max_alloc = Some(1024 * 1024 * 1024);
            reader.limits(limits);

            reader.with_guessed_format()?.decode()?
        }
    };

    Ok(dyn_img)
}
