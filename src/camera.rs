use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;

pub struct Camera {
    pub position: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    half_height: f32,
    half_width: f32,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, world_up: Vec3, fov: f32, aspect_ratio: f32) -> Self {
        let forward = (look_at - position).normalize();
        let right = forward.cross(&world_up).normalize();
        let up = right.cross(&forward).normalize();

        let half_height = (fov * 0.5).to_radians().tan();
        let half_width = half_height * aspect_ratio;

        Self {
            position,
            forward,
            right,
            up,
            half_height,
            half_width,
        }
    }

    pub fn make_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let u = 2.0 * (x as f32 + 0.5) / width as f32 - 1.0;
        let v = 1.0 - 2.0 * (y as f32 + 0.5) / height as f32;

        let direction = (self.forward 
            + self.right * (u * self.half_width) 
            + self.up * (v * self.half_height))
            .normalize();

        Ray {
            origin: self.position,
            direction,
        }
    }
}