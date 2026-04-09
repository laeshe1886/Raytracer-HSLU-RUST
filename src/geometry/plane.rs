use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::geometry::hittable::{Hit, Hittable};
use crate::material::Material;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Hittable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() > 0.0001 {
            let t = (self.point - ray.origin).dot(&self.normal) / denom;
            if t > 0.001 {
                return Some(Hit {
                    distance: t,
                    point: ray.at(t),
                    normal: if denom < 0.0 { self.normal } else { self.normal * -1.0 },
                    uv: (0.0, 0.0),
                    material: self.material,
                });
            }
        }
        None
    }
}