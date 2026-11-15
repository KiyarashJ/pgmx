pub fn encoder(extracted_pixels: &[u8]) -> (Vec<u8>, Vec<i16>, Vec<i16>, Vec<u8>) {
    println!("this is length of pixels : {}", extracted_pixels.len());
    let vector_capacity = extracted_pixels.len() / 4;
    let mut y_vector: Vec<u8> = Vec::with_capacity(vector_capacity);
    let mut co_vector: Vec<i16> = Vec::with_capacity(vector_capacity);
    let mut cg_vector: Vec<i16> = Vec::with_capacity(vector_capacity);
    let mut alpha_channels: Vec<u8> = Vec::with_capacity(vector_capacity);
    for chunks in extracted_pixels.chunks_exact(4) {
        let r = chunks[0] as i16;
        let g = chunks[1] as i16;
        let b = chunks[2] as i16;
        let a = chunks[3];
        let co = r - b;
        let t  = b + (co >> 1);
        let cg = g - t;
        let y  = t + (cg >> 1);
        y_vector.push(y as u8);
        co_vector.push(co);
        cg_vector.push(cg);
        alpha_channels.push(a)
    }
    (y_vector , co_vector, cg_vector, alpha_channels)
}
