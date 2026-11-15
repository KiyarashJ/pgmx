use std::{fs::File, io::{Error, Write}};

use bytemuck::bytes_of;
use image::EncodableLayout;

use crate::{structs::r#struct::Pgmxheader};

pub fn create(
    file_ext: [u8; 4], datas: (Vec<u8>, Vec<i16>, Vec<i16>, Vec<u8>), width: u32, height: u32)
    -> Result<String, Error>
    {
        let mut create_file = File::create("output.pgmx")?;
                let header = Pgmxheader::new(width, height, 1, file_ext);
                let _ = create_file.write_all(bytes_of(&header))?;
                let _ = create_file.write_all(datas.0.as_bytes())?;
                for co in &datas.1 {
                    create_file.write_all(&co.to_le_bytes())?;
                }
                for cg in &datas.2 {
                    create_file.write_all(&cg.to_le_bytes())?;
                }
                let _ = create_file.write_all(datas.3.as_bytes())?;
                println!("pgmx header while encoding : {:?}", header);
                
    Ok("Writing pgmx file and changing format was successful".to_string())
}