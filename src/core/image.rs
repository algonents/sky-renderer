use image::{ImageReader};


// core/image.rs
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA8 format
}


pub fn load_image(path: &str) -> Image {
    let img = ImageReader::open(path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .to_rgba8();

    let (width, height) = img.dimensions();
    let pixels = img.into_raw();

    Image {
        width,
        height,
        pixels,
    }
}