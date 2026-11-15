use crate::structs::r#struct::Pgmxheader;

pub fn get_header_and_datas(data: Vec<u8>) -> (Pgmxheader, Vec<u8>) {
    let header_size = std::mem::size_of::<Pgmxheader>();
    let (header, pixels) = data.split_at(header_size);
    let header_pgmx = unsafe {
        std::ptr::read_unaligned(header.as_ptr() as *const Pgmxheader)
    };
    let pixels_vec = pixels.to_vec();
    (header_pgmx, pixels_vec)
}