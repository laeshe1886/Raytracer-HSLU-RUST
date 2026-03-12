use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::geometry::hittable::{Hit, Hittable};

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

impl Hittable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let denom = ray.direction.dot(&self.normal);
        
        if denom.abs() > 1e-6 {
            let v = self.point - ray.origin;
            let lambda = v.dot(&self.normal) / denom;
            
            if lambda > 0.001 {
                let hit_point = ray.origin + ray.direction * lambda;
                return Some(Hit { 
                    distance: lambda, 
                    point: hit_point, 
                    normal: self.normal, 
                    color: self.color 
                });
            }
        }
        None
    }
}