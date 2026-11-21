<div align="center">

# 🖼️ pgmx

### *PGM, but it remembers your colors.*

**An extended PGM format that gives you a grayscale image on the surface and keeps the original color data safe underneath.**

[![Crates.io](https://img.shields.io/crates/v/pgmx.svg)](https://crates.io/crates/pgmx)
[![Docs.rs](https://img.shields.io/docsrs/pgmx)](https://docs.rs/pgmx)
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg)](#-license)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-MVP-yellow.svg)](#-roadmap)

</div>

---

## 🤔 Why does pgmx exist?

When you convert a color image to grayscale (classic PGM), the color information is **gone forever**.

**pgmx** fixes that. It stores the image in a luma/chroma layout:

| Channel | Meaning |
|---------|---------|
| **Y**   | Luma, which *is* your grayscale image 🖤🤍 |
| **Co**  | Chroma orange |
| **Cg**  | Chroma green |
| *(optional)* **A** | Alpha |

The Y channel sits right at the start of the data section, so you can show a grayscale preview **without decoding the color channels**, and still convert back to the original color image whenever you want. 🎨

> 💡 **In one sentence:** grayscale when you need it, color when you want it back.

---

## ✨ Features

- 🔁 **Two-way conversion**: regular image → `.pgmx` → regular image
- 🧩 **Simple fixed-size header**: easy to parse from any language
- 🖤 **Instant grayscale access**: Y channel is directly readable
- 🛡️ **Error handling with `Result`/`Option`**: corrupted files don't panic
- 🖥️ **Separate viewer app** (built with [Dioxus](https://dioxuslabs.com/)) to open `.pgmx` files
- 🦀 **Pure Rust**, no unsafe code required for users of the library

---

## 📦 Installation

```toml
[dependencies]
pgmx = "0.1.1"
```

or

```bash
cargo add pgmx
```

---

## 🚀 Quick start


### Convert a normal image to `.pgmx`

```rust
use pgmx::encoder_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PNG / JPEG / WebP -> PGMX
    encoder_handler::encoder_handler("photo.png");
    Ok(())
}
```

### Convert `.pgmx` back to the original format

```rust
use pgmx::decoder_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PGMX -> original format (stored in the header)
    decoder_handler::decoder_handler("photo.pgmx");
    Ok(())
}
```
```

---

## 🧱 File format

A `.pgmx` file is a **fixed-size header** followed by **raw pixel data**.

### Header layout (38 bytes, packed, little-endian)

| Offset | Size | Field | Description |
|-------:|-----:|-------|-------------|
| 0  | 4  | `width`           | Image width in pixels (`u32`) |
| 4  | 4  | `height`          | Image height in pixels (`u32`) |
| 8  | 4  | `magic`           | File signature, used to verify it's a pgmx file |
| 12 | 2  | `max_val`         | Maximum pixel value (`255` for 8-bit) |
| 14 | 1  | `channels`        | Number of color channels (Y, Co, Cg → `3`) |
| 15 | 1  | `color_transform` | Whether the color transform was applied |
| 16 | 1  | `data_layout`     | `0` = planar (Y, then Co, then Cg), `1` = interleaved |
| 17 | 15 | `reserved`        | Reserved for future use |
| 32 | 1  | `version`         | Format version |
| 33 | 1  | `alpha_channels`  | Number of alpha channels |
| 34 | 4  | `format_ext`      | Original file extension (`png\0`, `jpeg`, `webp`, ...) |

### Data section

After the header, pixels follow in the order set by `data_layout`:

```
planar (0):       [ Y Y Y ... ][ Co Co Co ... ][ Cg Cg Cg ... ]
interleaved (1):  [ Y Co Cg ][ Y Co Cg ] ...
```

> 📝 Data is currently stored **uncompressed**. See the [roadmap](#-roadmap).

---

## 🖥️ The viewer

The repository also ships a small desktop/web viewer built with Dioxus:

1. Open the app
2. Choose a `.pgmx` file
3. See it rendered in grayscale 🖤🤍

```bash
cd viewer
dx serve
```

The viewer depends on the core library, but the core library **does not** depend on Dioxus, so you can use `pgmx` in CLI tools, servers, or anywhere else.

---

## ⚠️ Known limitations

Being honest about the MVP:

- 📦 **No compression yet.** Files are roughly `width × height × 3` bytes, so a 40 KB JPEG can become several MB. (A JPEG is already compressed. pgmx currently stores raw pixels.)
- 🎞️ **JPEG output is lossy.** Converting back to JPEG re-encodes the pixels, so the result is visually the same but not byte-identical to the original file. PNG round-trips exactly.
- 🔢 **Chroma precision.** Fully reversible YCoCg-R needs 9 bits for Co and Cg. Check the current implementation notes before relying on bit-exact round trips for every pixel.
- 🌐 **WebP output** requires the `webp` feature of the `image` crate.

---

## 🗺️ Roadmap

- [x] Header + encoder + decoder
- [x] Two-way conversion
- [x] Viewer app
- [ ] 🗜️ Lossless compression (zstd / deflate per channel)
- [ ] 🧪 Round-trip test suite with sample images
- [ ] 💻 CLI tool: `pgmx convert in.png out.pgmx`
- [ ] 🎨 Show full color in the viewer (toggle gray / color)
- [ ] 📄 Formal `SPEC.md`
- [ ] 🔢 Numeric format codes instead of extension strings
- [ ] 🌈 16-bit support

---

## 🤝 Contributing

This is my first big Rust project, so feedback is very welcome! 🦀

1. Fork the repo
2. Create a branch: `git checkout -b feature/cool-thing`
3. Commit your changes
4. Open a Pull Request

Bug reports and ideas are welcome in the [Issues](https://github.com/YOUR_USERNAME/pgmx/issues) tab.

---

## 📜 License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

<div align="center">

Made with 🦀 and a lot of ☕

**If you like pgmx, give it a ⭐ on GitHub!**

</div>
