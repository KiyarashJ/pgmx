use bytemuck::{Pod, Zeroable};


#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Pgmxheader {
    pub width: u32, // Photo width
    pub height: u32, // Photo height
    pub magic: [u8; 4], // Signature of file , checks whether the photo Format is pgmx or not !
    pub max_val : u16, // Max value of pixels that can have ! is 255 !
    pub channels: u8, // this has 3 channels , y , Cg , Co.
    pub color_transform : u8, // Is this photo transformed to pgmx ?
    pub data_layout : u8, // Sequence of sorting datas. 0 means planar (first off Y , then Co , then Cg), 1 means interleaved.
    pub reserved: [u8; 15], // reserved Storage for later , if you want to add new field to header of Photo you can use this !
    pub version: u8,
    pub alpha_channels: u8,
    pub format_ext: [u8; 4] // version of format ! how many times is this file tampered !
}