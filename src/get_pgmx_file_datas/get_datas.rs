use std::{fs::File, io::{Error, Read}};

use crate::get_format_ext::get_ext::get_extension_format;

pub fn get_pgmx_header(file: &str) -> Result<Vec<u8>, Error> {
    if get_extension_format(file).as_str().trim() == "pgmx" {
        let mut open = File::open(file)?;
        let mut file_data: Vec<u8> = Vec::new();
        let _ = open.read_to_end(&mut file_data)?;
        Ok(file_data)
    } else {
        panic!("This format is not *.pgmx format !")
    }
}