use raylib::prelude::*;

use crate::ray_intersect::{Ray, Vec3};

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub texture_a: Color,
    pub texture_b: Color,
    pub texture_scale: f32,
}

impl Cube {
    pub fn new(
        center: Vec3,
        size: f32,
        texture_a: Color,
        texture_b: Color,
    ) -> Self {
        let half_size = size * 0.5;
        let offset = Vec3::new(half_size, half_size, half_size);

        Self {
            min: center - offset,
            max: center + offset,
            texture_a,
            texture_b,
            texture_scale: 8.0,
        }
    }

    pub fn texture_color(&self, point: Vec3, normal: Vec3) -> Color {
        let (u, v) = if normal.x.abs() > 0.5 {
            (point.z, point.y)
        } else if normal.y.abs() > 0.5 {
            (point.x, point.z)
        } else {
            (point.x, point.y)
        };

        let tile_u = ((u - self.min.x) * self.texture_scale).floor() as i32;
        let tile_v = ((v - self.min.y) * self.texture_scale).floor() as i32;

        if (tile_u + tile_v) % 2 == 0 {
            self.texture_a
        } else {
            self.texture_b
        }
    }

    pub fn texture_color_at(&self, point: Vec3, normal: Vec3) -> Color {
        let band = 0.26;
        let height = self.max.y - self.min.y;

        if point.y > self.max.y - band * height || point.y < self.min.y + band * height {
            return if ((point.x * self.texture_scale).floor()
                + (point.z * self.texture_scale).floor()) as i32
                % 2
                == 0
            {
                Color::new(190, 34, 25, 255)
            } else {
                Color::new(110, 20, 18, 255)
            };
        }

        if normal.y.abs() < 0.5 {
            let stripe = ((point.x + point.z) * self.texture_scale).floor() as i32;
            return if stripe % 2 == 0 {
                Color::new(220, 220, 215, 255)
            } else {
                Color::new(55, 60, 72, 255)
            };
        }

        self.texture_color(point, normal)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(f32, Vec3)> {
        let mut near = 0.001;
        let mut far = f32::INFINITY;
        let mut near_normal = Vec3::new(0.0, 0.0, 0.0);
        let axes = [
            (ray.origin.x, ray.direction.x, self.min.x, self.max.x, Vec3::new(1.0, 0.0, 0.0)),
            (ray.origin.y, ray.direction.y, self.min.y, self.max.y, Vec3::new(0.0, 1.0, 0.0)),
            (ray.origin.z, ray.direction.z, self.min.z, self.max.z, Vec3::new(0.0, 0.0, 1.0)),
        ];

        for (origin, direction, min, max, axis_normal) in axes {
            if direction.abs() < f32::EPSILON {
                if origin < min || origin > max {
                    return None;
                }
                continue;
            }

            let mut t_min = (min - origin) / direction;
            let mut t_max = (max - origin) / direction;
            let mut normal = axis_normal * -1.0;

            if t_min > t_max {
                std::mem::swap(&mut t_min, &mut t_max);
                normal = axis_normal;
            }

            if t_min > near {
                near = t_min;
                near_normal = normal;
            }

            far = far.min(t_max);

            if near > far {
                return None;
            }
        }

        Some((near, near_normal))
    }
}
