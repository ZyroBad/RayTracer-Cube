mod cube;
mod framebuffer;
mod ray_intersect;

use cube::Cube;
use framebuffer::Framebuffer;
use ray_intersect::{Ray, Vec3};
use raylib::prelude::*;

fn rotate_x(vector: Vec3, angle: f32) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    Vec3::new(
        vector.x,
        vector.y * cos - vector.z * sin,
        vector.y * sin + vector.z * cos,
    )
}

fn rotate_y(vector: Vec3, angle: f32) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    Vec3::new(
        vector.x * cos + vector.z * sin,
        vector.y,
        -vector.x * sin + vector.z * cos,
    )
}

fn rotate_cube_to_world(vector: Vec3, rotation_x: f32, rotation_y: f32) -> Vec3 {
    rotate_y(rotate_x(vector, rotation_x), rotation_y)
}

fn rotate_world_to_cube_with_angles(
    vector: Vec3,
    rotation_x: (f32, f32),
    rotation_y: (f32, f32),
) -> Vec3 {
    let (sin_y, cos_y) = rotation_y;
    let (sin_x, cos_x) = rotation_x;
    let rotated_y = Vec3::new(
        vector.x * cos_y - vector.z * sin_y,
        vector.y,
        vector.x * sin_y + vector.z * cos_y,
    );

    Vec3::new(
        rotated_y.x,
        rotated_y.y * cos_x + rotated_y.z * sin_x,
        -rotated_y.y * sin_x + rotated_y.z * cos_x,
    )
}

fn render_cube(
    framebuffer: &mut Framebuffer,
    rotation_x: f32,
    rotation_y: f32,
) {
    let cube = Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.2,
        Color::new(235, 235, 235, 255),
        Color::new(35, 75, 135, 255),
    );
    let light_position = Vec3::new(-1.5, 1.8, 2.5);
    let camera_origin = Vec3::new(2.8, 1.8, 6.0);
    let rotation_x_angles = (-rotation_x).sin_cos();
    let rotation_y_angles = (-rotation_y).sin_cos();
    let local_origin = rotate_world_to_cube_with_angles(
        camera_origin,
        rotation_x_angles,
        rotation_y_angles,
    );

    let aspect = framebuffer.width() as f32 / framebuffer.height() as f32;

    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let ndc_x = (x as f32 / framebuffer.width() as f32) * 2.0 - 1.0;
            let ndc_y = 1.0 - (y as f32 / framebuffer.height() as f32) * 2.0;

            let screen_point = Vec3::new(ndc_x * aspect, ndc_y, 0.0);
            let ray_dir = (screen_point - camera_origin).normalize();
            let local_direction = rotate_world_to_cube_with_angles(
                ray_dir,
                rotation_x_angles,
                rotation_y_angles,
            );
            let ray = Ray::new(local_origin, local_direction);

            if let Some((t, normal)) = cube.intersect(&ray) {
                let local_hit_point = ray.origin + ray.direction * t;
                let hit_point = rotate_cube_to_world(local_hit_point, rotation_x, rotation_y);
                let world_normal = rotate_cube_to_world(normal, rotation_x, rotation_y).normalize();
                let light_direction = (light_position - hit_point).normalize();
                let diffuse = world_normal.dot(light_direction).max(0.15);
                let texture_color = cube.texture_color_at(local_hit_point, normal);

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

    let mut rotation_x = -0.12;
    let mut rotation_y = 0.0;
    let mut needs_render = true;

    while !window.window_should_close() {
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();

        if current_width > 0 && current_height > 0 {
            if framebuffer.width() != current_width || framebuffer.height() != current_height {
                framebuffer = Framebuffer::new(current_width, current_height);
                framebuffer.set_background_color(Color::BLACK);
                framebuffer.clear();
                needs_render = true;
            }

            if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                let mouse_delta = window.get_mouse_delta();
                if mouse_delta.x != 0.0 || mouse_delta.y != 0.0 {
                    rotation_y += mouse_delta.x * 0.01;
                    rotation_x += mouse_delta.y * 0.01;
                    rotation_x = rotation_x.clamp(-1.3, 1.3);
                    needs_render = true;
                }
            }

            let frame_changed = needs_render;

            if frame_changed {
                render_cube(&mut framebuffer, rotation_x, rotation_y);
                needs_render = false;
            }

            framebuffer.swap_buffers(&mut window, &raylib_thread, frame_changed);
        } else {
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
