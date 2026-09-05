use raylib::ffi;
use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    texture: Option<Texture2D>,
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
            texture: None,
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

    pub fn swap_buffers(
        &mut self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        update_texture: bool,
    ) {
        if self.texture.is_none() {
            match window.load_texture_from_image(raylib_thread, &self.image) {
                Ok(texture) => self.texture = Some(texture),
                Err(error) => {
                    eprintln!(
                        "No se pudo cargar el framebuffer en la ventana: {:?}",
                        error
                    );
                    return;
                }
            }
        }

        if let Some(texture) = self.texture.as_mut() {
            if update_texture {
                let pixels = self.image.get_image_data();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        pixels.as_ptr() as *const u8,
                        pixels.len() * std::mem::size_of::<Color>(),
                    )
                };

                if let Err(error) = texture.update_texture(bytes) {
                    eprintln!(
                        "No se pudo actualizar el framebuffer en la ventana: {:?}",
                        error
                    );
                }
            }

            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.clear_background(self.background_color);
            renderer.draw_texture(texture, 0, 0, Color::WHITE);
        }
    }
}
