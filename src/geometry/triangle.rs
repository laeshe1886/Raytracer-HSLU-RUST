use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::geometry::hittable::{Hit, Hittable};
use crate::material::Material;

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub material: Material,
}

impl Hittable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a > -0.00001 && a < 0.00001 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.a;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);
        if t > 0.001 {
            let outward_normal = edge1.cross(&edge2).normalize();
            let frontface = ray.direction.dot(&outward_normal) < 0.0;
            let normal = if frontface { outward_normal } else { outward_normal * -1.0 };

            return Some(Hit {
                distance: t,
                point: ray.at(t),
                normal,
                uv: (0.0, 0.0),
                material: self.material,
                frontface,
            });
        }
        None
    }
}