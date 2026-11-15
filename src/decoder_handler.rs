use std::{io::{Error}};

// use image::ImageError;

use crate::{from_pgmx_algo::{from_pgmx::from_pgmx, image_build::build_img}, get_pgmx_file_datas::{get_datas::get_pgmx_header, get_header_and_datas::get_header_and_datas}};

pub fn decoder_handler(file: &str) -> Result<(), Error> {
    let data_files = match get_pgmx_header(file) {
        Ok(file) =>  file,
        Err(error) => {
        println!("There is an error about getting header of pgmx file");
        return Err(error)
    }
    };
    let (header, pixels) = get_header_and_datas(data_files);
    let pgmx_reformat = match from_pgmx(header, pixels) {
        Ok(reformat) => reformat,
        Err(error) => {
        println!("There is an error with reformatting PGMX!");

        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            error,
        ));
    }
    };
    let _ = build_img(pgmx_reformat, header);
    Ok(())
}