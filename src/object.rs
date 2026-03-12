use crate::vector3d::Vec3;
use crate::ray::Ray;

pub struct Hit {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

pub enum Object {
    Sphere { center: Vec3, radius: f32, color: Vec3 },
    Plane { point: Vec3, normal: Vec3, color: Vec3 },
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Object::Sphere { center, radius, color } => {
                let v = ray.origin - *center;
                let a = ray.direction.dot(&ray.direction);
                let b = 2.0 * ray.direction.dot(&v);
                let c = v.dot(&v) - radius * radius;
                
                let discriminant = b * b - 4.0 * a * c;
                if discriminant >= 0.0 {
                    let lambda = (-b - discriminant.sqrt()) / (2.0 * a);
                    if lambda > 0.001 {
                        let hit_point = ray.origin + ray.direction * lambda;
                        let normal = (hit_point - *center).normalize();
                        return Some(Hit { distance: lambda, point: hit_point, normal, color: *color });
                    }
                }
                None
            }
            Object::Plane { point, normal, color } => {
                let denom = ray.direction.dot(normal);
                if denom.abs() > 1e-6 {
                    let v = *point - ray.origin;
                    let lambda = v.dot(normal) / denom;
                    if lambda > 0.001 {
                        let hit_point = ray.origin + ray.direction * lambda;
                        return Some(Hit { distance: lambda, point: hit_point, normal: *normal, color: *color });
                    }
                }
                None
            }
        }
    }
}