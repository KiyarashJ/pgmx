
use pgmx::{decoder_handler, encoder_handler};
use image::ImageReader;

#[test]
fn test_pgmx_lossless_roundtrip() {
    let original_pixels: Vec<u8> = vec![
        255, 0, 0, 255,
        0, 255, 0, 255,
        0, 0, 255, 255,
        255, 255, 255, 255,
    ];

    let original_image =
        image::RgbaImage::from_raw(2, 2, original_pixels.clone()).unwrap();

    let input_path = std::env::temp_dir().join("pgmx_test_input.png");

    original_image.save(&input_path).unwrap();

    // Encode
    let pgmx_path = encoder_handler(
        input_path.to_str().unwrap()
    ).expect("Encoding failed");

    // Decode
    decoder_handler(&pgmx_path).expect("Decoding failed");

    // Read decoded image
    let decoded_image = ImageReader::open("output_reformat.png")
        .expect("Cannot open decoded image")
        .decode()
        .expect("Cannot decode output image")
        .to_rgba8();

    // Compare original and decoded pixels
    assert_eq!(
        original_pixels,
        decoded_image.into_raw(),
        "PGMX is not lossless!"
    );

    // Cleanup
    std::fs::remove_file(input_path).ok();
    std::fs::remove_file(pgmx_path).ok();
    std::fs::remove_file("output_reformat.png").ok();
}
