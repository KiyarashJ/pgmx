use crate::structs::r#struct::Pgmxheader;

impl Pgmxheader {
    pub fn new(width: u32, height: u32, alpha_channels: u8, format_ext: [u8; 4]) -> Pgmxheader {
        Self {
            width,
            height,
            magic: *b"PGMX",
            max_val: 255,
            channels: 3,
            color_transform: 1,
            data_layout: 0,
            reserved: [0; 15],
            version: 1,
            alpha_channels,
            format_ext
        }
    }
}