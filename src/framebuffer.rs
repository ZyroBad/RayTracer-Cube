use raylib::ffi;
use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    width: i32,
    height: i32,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;

        let image = unsafe { Image::from_raw(ffi::GenImageColor(width, height, background_color)) };

        Self {
            image,
            width,
            height,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn clear(&mut self) {
        let new_image = unsafe {
            Image::from_raw(ffi::GenImageColor(
                self.width,
                self.height,
                self.background_color,
            ))
        };

        self.image = new_image;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }

        self.image.draw_pixel(x, y, self.current_color);
    }

    pub fn pixel_at(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        let pixels = self.image.get_image_data();
        let index = y as usize * self.width as usize + x as usize;

        let pixel = pixels[index];
        Some(pixel)
    }

    pub fn render_to_file(&self, file_name: &str) {
        self.image.export_image(file_name);

        println!("Proceso de exportación terminado: {}", file_name);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        let texture_result = window.load_texture_from_image(raylib_thread, &self.image);

        match texture_result {
            Ok(texture) => {
                let mut renderer = window.begin_drawing(raylib_thread);

                renderer.clear_background(self.background_color);

                renderer.draw_texture(&texture, 0, 0, Color::WHITE);
            }

            Err(error) => {
                eprintln!(
                    "No se pudo cargar el framebuffer en la ventana: {:?}",
                    error
                );
            }
        }
    }
}
