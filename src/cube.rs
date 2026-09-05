use raylib::prelude::*;

use crate::ray_intersect::{Ray, Vec3};

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub color: Color,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, color: Color) -> Self {
        let half_size = size * 0.5;
        let offset = Vec3::new(half_size, half_size, half_size);

        Self {
            min: center - offset,
            max: center + offset,
            color,
        }
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
