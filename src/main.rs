mod cube;
mod framebuffer;
mod ray_intersect;

use cube::Cube;
use framebuffer::Framebuffer;
use ray_intersect::{Ray, Vec3};
use raylib::prelude::*;

fn render_cube(framebuffer: &mut Framebuffer) {
    let cube = Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.2,
        Color::new(235, 235, 235, 255),
        Color::new(35, 75, 135, 255),
    );
    let light_position = Vec3::new(-1.5, 1.8, 2.5);
    let camera_origin = Vec3::new(2.8, 1.8, 6.0);

    let aspect = framebuffer.width() as f32 / framebuffer.height() as f32;

    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let ndc_x = (x as f32 / framebuffer.width() as f32) * 2.0 - 1.0;
            let ndc_y = 1.0 - (y as f32 / framebuffer.height() as f32) * 2.0;

            let screen_point = Vec3::new(ndc_x * aspect, ndc_y, 0.0);
            let ray_dir = (screen_point - camera_origin).normalize();
            let ray = Ray::new(camera_origin, ray_dir);

            if let Some((t, normal)) = cube.intersect(&ray) {
                let hit_point = ray.origin + ray.direction * t;
                let light_direction = (light_position - hit_point).normalize();
                let diffuse = normal.dot(light_direction).max(0.15);
                let texture_color = cube.texture_color(hit_point, normal);

                let red = texture_color.r as f32 * diffuse;
                let green = texture_color.g as f32 * diffuse;
                let blue = texture_color.b as f32 * diffuse;

                framebuffer.set_current_color(Color::new(
                    red.clamp(0.0, 255.0) as u8,
                    green.clamp(0.0, 255.0) as u8,
                    blue.clamp(0.0, 255.0) as u8,
                    255,
                ));
                framebuffer.point(x, y);
            } else {
                framebuffer.set_current_color(Color::new(12, 17, 25, 255));
                framebuffer.point(x, y);
            }
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 450;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracing difuso: cubo texturizado")
        .resizable()
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    while !window.window_should_close() {
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();

        if current_width > 0 && current_height > 0 {
            if framebuffer.width() != current_width || framebuffer.height() != current_height {
                framebuffer = Framebuffer::new(current_width, current_height);
                framebuffer.set_background_color(Color::BLACK);
                framebuffer.clear();
            }

            render_cube(&mut framebuffer);
            framebuffer.swap_buffers(&mut window, &raylib_thread);
        } else {
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
