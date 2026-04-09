use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::geometry::hittable::{Hit, Hittable};
use crate::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / (2.0 * a);
            if temp > 0.001 {
                let point = ray.at(temp);
                let normal = (point - self.center).normalize();
                return Some(Hit {
                    distance: temp,
                    point,
                    normal,
                    uv: (0.0, 0.0),
                    material: self.material,
                });
            }
            let temp = (-b + discriminant.sqrt()) / (2.0 * a);
            if temp > 0.001 {
                let point = ray.at(temp);
                let normal = (point - self.center).normalize();
                return Some(Hit {
                    distance: temp,
                    point,
                    normal,
                    uv: (0.0, 0.0),
                    material: self.material,
                });
            }
        }
        None
    }
}