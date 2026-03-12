use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::geometry::hittable::{Hit, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Vec3,
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let v = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&v);
        let c = v.dot(&v) - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let lambda = (-b - discriminant.sqrt()) / (2.0 * a);
            if lambda > 0.001 {
                let hit_point = ray.origin + ray.direction * lambda;
                let normal = (hit_point - self.center).normalize();
                return Some(Hit { 
                    distance: lambda, 
                    point: hit_point, 
                    normal, 
                    color: self.color 
                });
            }
        }
        None
    }
}