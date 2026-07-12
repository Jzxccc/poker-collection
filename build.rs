use image::{RgbaImage, Rgba, ImageEncoder};
use image::codecs::ico::IcoEncoder;
use std::io::Cursor;
use std::path::PathBuf;

fn generate_default_icon() -> Vec<u8> {
    let size = 32u32;
    let mut img = RgbaImage::new(size, size);
    for y in 0..size {
        for x in 0..size {
            let cx = x as f32 - 16.0;
            let cy = y as f32 - 16.0;
            let in_rect = cx.abs() < 14.0 && cy.abs() < 14.5;
            if in_rect {
                let h = cy / 14.0;
                let w = cx / 14.0;
                if h < -0.2 && h > -0.95 && w.abs() < (-h - 0.2) * 0.8 {
                    img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                } else {
                    img.put_pixel(x, y, Rgba([34, 94, 44, 255]));
                }
            } else {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }
    let mut buf = Cursor::new(Vec::new());
    IcoEncoder::new(&mut buf).write_image(img.as_raw(), 32, 32, image::ColorType::Rgba8.into()).unwrap();
    buf.into_inner()
}

/// Convert any image to ICO format, resizing to max 256x256 if needed.
fn png_to_ico(path: &PathBuf) -> Vec<u8> {
    use image::imageops::FilterType;
    let data = std::fs::read(path).unwrap();
    let img = image::load_from_memory(&data).unwrap().to_rgba8();
    let (w, h) = img.dimensions();
    let max_dim = 256;
    let img = if w > max_dim || h > max_dim {
        let ratio = max_dim as f32 / w.max(h) as f32;
        let nw = (w as f32 * ratio) as u32;
        let nh = (h as f32 * ratio) as u32;
        image::imageops::resize(&img, nw, nh, FilterType::Lanczos3)
    } else {
        img
    };
    let (w, h) = img.dimensions();
    let mut buf = Cursor::new(Vec::new());
    IcoEncoder::new(&mut buf).write_image(img.as_raw(), w, h, image::ColorType::Rgba8.into()).unwrap();
    buf.into_inner()
}

fn main() {
    println!("cargo:rerun-if-changed=icon.png");
    println!("cargo:rerun-if-changed=icon.jpg");
    println!("cargo:rerun-if-changed=icon.jpeg");
    println!("cargo:rerun-if-changed=build.rs");

    let project_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let png_icon = project_root.join("icon.png");
    let jpg_icon = project_root.join("icon.jpg");
    let jpeg_icon = project_root.join("icon.jpeg");

    let user_icon = if png_icon.exists() { png_icon }
        else if jpg_icon.exists() { jpg_icon }
        else if jpeg_icon.exists() { jpeg_icon }
        else { PathBuf::new() };

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let ico_path = std::path::Path::new(&out_dir).join("poker_icon.ico");

    if user_icon.exists() {
        println!("cargo:warning=Using custom icon: {}", user_icon.display());
        std::fs::write(&ico_path, png_to_ico(&user_icon)).unwrap();
    } else {
        println!("cargo:warning=No icon found, using default. Place icon.png/icon.jpg in project root.");
        std::fs::write(&ico_path, generate_default_icon()).unwrap();
    }

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon(ico_path.to_str().unwrap());
        res.compile().unwrap();
    }
}
