use std::collections::HashMap;
use std::os::raw::c_void;
use std::path::Path;

use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, ColorType, Rgba};
use imageproc::drawing::draw_text_mut;

#[allow(dead_code)]
pub struct ImageManager {
    image_map: HashMap<String, u32>
}

#[allow(dead_code)]
impl ImageManager {
    pub fn new() -> ImageManager {
        let image_manager = ImageManager {
            image_map: HashMap::new()
        };
        image_manager
    }

    pub fn load_image(&mut self, path: &str, id: &str, vflip: bool) -> bool {
        let path = Path::new(path);

        if !path.exists() {
            return false;
        }

        let mut image = image::open(path).expect("failed to load image");

        self.insert_image(&mut image, id, vflip)
    }

    pub fn write_text(&mut self, text: &str, h: u32, w: u32, r: u8, g: u8, b: u8, id: &str) -> bool {
        let mut image = DynamicImage::new(h, w * text.len() as u32, ColorType::Rgba8);
        let font = FontRef::try_from_slice(include_bytes!("../rsc/fonts/ipag00303/ipag.ttf")).unwrap();
        let scale = PxScale { x: w as f32, y: h as f32 };

        draw_text_mut(&mut image, Rgba([r, g, b, 255u8]), 0, 0, scale, &font, text);

        let mut image = image.fliph();
        self.insert_image(&mut image, id, true)
    }

    pub fn insert_image(&mut self, image: &mut DynamicImage, id: &str, vflip: bool) -> bool {
        let format = match image {
            DynamicImage::ImageLuma8(_) => gl::RED,
            DynamicImage::ImageLumaA8(_) => gl::RG,
            DynamicImage::ImageRgb8(_) => gl::RGB,
            DynamicImage::ImageRgba8(_) => gl::RGBA,
            _ => gl::RGBA
        };

        if vflip {
            *image = image.flipv();
        }

        let data = image.as_bytes();
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        self.image_map.insert(id.to_string(), texture);

        true
    }

    pub fn delete_texture(&mut self, id: &str) {
        let _id = *self.image_map.get(id).expect("failed to get texture");
        unsafe {
            gl::DeleteTextures(1, &_id);
        }
        self.image_map.remove(id);
    }

    pub fn get_texture_id(&mut self, id: &str) -> u32 {
        *self.image_map.get(id).expect("failed to get texture")
    }
}
