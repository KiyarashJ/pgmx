use image::ImageError;
use crate::{create_file::file_creation::create, get_format_ext::get_ext::get_extension_format, get_image_pixels::extracting_pixels::{extract_pixel}, to_pgmx_algo::y_co_cg_algo::encoder};

pub fn encoder_handler(path: &str) -> Result<String , ImageError> {
    let (extract_pixels, width, height) = match extract_pixel(path){
        Ok((extracting_pixels, width, height)) => (extracting_pixels, width, height),
        Err(error) => {
            println!("there is an error about extracting pixels");
            return Err(error)
        }
    };
    let extract_file_extension = get_extension_format(path);
    let encoder_to_pgmx: (Vec<u8>, Vec<i16>, Vec<i16>, Vec<u8>) = encoder(extract_pixels.as_slice());
    let ext = extract_file_extension.as_bytes();
    let mut ext_bytes = [0u8; 4];
    ext_bytes[..ext.len()].copy_from_slice(ext);
    let make_pgmx_file = create(ext_bytes, encoder_to_pgmx, width, height);
    
    Ok(make_pgmx_file?)
}