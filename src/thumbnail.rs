use fast_image_resize::{FilterType, ResizeAlg, ResizeOptions, Resizer};
use image::{DynamicImage, GenericImageView};

use crate::error::RustyNailerError;


/// Resizes the given image using the provided max pixel size for its smallest dimension,
/// and returns the scaled-down image. Uses a fast filter (Triangle) for downsizing.
pub fn resize_image(
    img: &DynamicImage,
    max_dimension: u32,
) -> Result<DynamicImage, RustyNailerError> {
    let (width, height) = img.dimensions();
    if width == 0 || height == 0 {
        return Err(RustyNailerError::Image(image::ImageError::Parameter(
            image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("Source image has no size.".into()),
            ),
        )));
    }

    let largest_side = width.max(height) as f32;
    let scale = max_dimension as f32 / largest_side;
    let dst_width = (width as f32 * scale).round() as u32;
    let dst_height = (height as f32 * scale).round() as u32;

    let mut dst_image = DynamicImage::new(dst_width, dst_height, img.color());

    let mut resizer = Resizer::new();

    resizer.resize(
        img,
        &mut dst_image,
        &ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::Box)),
    )?;

    Ok(dst_image)
}
