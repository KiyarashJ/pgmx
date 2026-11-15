use crate::structs::r#struct::Pgmxheader;

pub fn from_pgmx(
    header: Pgmxheader,
    pixels: Vec<u8>,
) -> Result<Vec<u8>, String> {

    let pixel_size = (header.width as usize)
        .checked_mul(header.height as usize)
        .ok_or("Image dimensions are too large")?;

    if header.data_layout != 0 {
        return Err("Unsupported data layout".to_string());
    }

    let alpha_size = if header.alpha_channels == 1 {
        pixel_size
    } else {
        0
    };

    let expected_size = pixel_size
        .checked_add(pixel_size * 2)
        .and_then(|v| v.checked_add(pixel_size * 2))
        .and_then(|v| v.checked_add(alpha_size))
        .ok_or("Pixel data is too large")?;

    if pixels.len() != expected_size {
        return Err("Invalid pixel data size".to_string());
    }

    let y_start = 0;
    let co_start = pixel_size;
    let cg_start = co_start + pixel_size * 2;
    let alpha_start = cg_start + pixel_size * 2;

    let y_vector = &pixels[y_start..co_start];
    let co_bytes = &pixels[co_start..cg_start];
    let cg_bytes = &pixels[cg_start..alpha_start];

    let alpha_vector = if header.alpha_channels == 1 {
        Some(&pixels[alpha_start..])
    } else {
        None
    };

    let mut rgba = Vec::with_capacity(pixel_size * 4);

    for i in 0..pixel_size {
        let y = y_vector[i] as i32;
        let co_index = i * 2;
        let co = i16::from_le_bytes([
            co_bytes[co_index],
            co_bytes[co_index + 1],
        ]) as i32;
        let cg_index = i * 2;
        let cg = i16::from_le_bytes([
            cg_bytes[cg_index],
            cg_bytes[cg_index + 1],
        ]) as i32;
        let t = y - (cg >> 1);
        let g = cg + t;
        let b = t - (co >> 1);
        let r = b + co;
        if !(0..=255).contains(&r)
            || !(0..=255).contains(&g)
            || !(0..=255).contains(&b)
        {
            return Err("Invalid color values in PGMX data".to_string());
        }

        let a = match alpha_vector {
            Some(alpha) => alpha[i],
            None => 255,
        };

        rgba.push(r as u8);
        rgba.push(g as u8);
        rgba.push(b as u8);
        rgba.push(a);
    }

    Ok(rgba)
}