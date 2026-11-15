use std::path::PathBuf;

pub fn get_extension_format(file: &str) -> String {
    let buffer: PathBuf = PathBuf::from(file);
    let get_ext = match buffer.extension() {
        Some(ext) => &ext.to_string_lossy(),
        _ => "There is something wrong with extracting extensions !"
    };
    get_ext.to_string()
}