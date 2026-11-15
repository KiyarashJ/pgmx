use image::{DynamicImage, ImageError, ImageFormat, RgbaImage};
use std::path::PathBuf;
use crate::structs::r#struct::Pgmxheader;

pub fn build_img(pixels: Vec<u8>, header: Pgmxheader) -> Result<(), ImageError> {
    let (width, height) = (header.width, header.height); 

    let rgba = RgbaImage::from_raw(width, height, pixels).ok_or_else(|| {
        ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        ))
    })?;

    let ext = std::str::from_utf8(&header.format_ext)
        .unwrap_or("png")
        .trim_end_matches('\0')
        .to_ascii_lowercase();

    let format = ImageFormat::from_extension(&ext).unwrap_or(ImageFormat::Png);
    let ext = if ImageFormat::from_extension(&ext).is_some() { ext } else { "png".to_string() };

    let img = DynamicImage::ImageRgba8(rgba);
    let img = if format == ImageFormat::Jpeg {
        DynamicImage::ImageRgb8(img.to_rgb8())
    } else {
        img
    };

    let path = PathBuf::from(format!("output_reformat.{ext}"));
    let _ = img.save_with_format(&path, format);
    Ok(())
}