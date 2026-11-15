use image::{ImageError, ImageReader};

pub fn extract_pixel(path: &str) -> Result <(Vec<u8>, u32, u32), ImageError> {
    let decode_img = ImageReader::open(path)?.decode()?;
    let width = decode_img.width();
    let height = decode_img.height();
    let rgb_values = decode_img.to_rgba8().into_raw();
    Ok((rgb_values, width, height))
}