use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::geometry::hittable::{Hit, Hittable};
use crate::material::Material;
use crate::geometry::aabb::AABB;

pub struct Triangle {
    pub a: Vec3, pub b: Vec3, pub c: Vec3,
    pub na: Option<Vec3>, pub nb: Option<Vec3>, pub nc: Option<Vec3>,
    pub material: Material,
}

impl Hittable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a > -0.00001 && a < 0.00001 { return None; }

        let f = 1.0 / a;
        let s = ray.origin - self.a;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 { return None; }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 { return None; }

        let t = f * edge2.dot(&q);
        if t > 0.001 {
            let w = 1.0 - u - v;
            let outward_normal = if let (Some(na), Some(nb), Some(nc)) = (self.na, self.nb, self.nc) {
                (na * w + nb * u + nc * v).normalize()
            } else {
                edge1.cross(&edge2).normalize() 
            };

            let frontface = ray.direction.dot(&outward_normal) < 0.0;
            let normal = if frontface { outward_normal } else { outward_normal * -1.0 };

            return Some(Hit {
                distance: t, point: ray.at(t), normal, uv: (u, v),
                material: self.material, frontface,
            });
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        let min = crate::math::vector3d::Vec3::new(
            self.a.x.min(self.b.x).min(self.c.x) - 0.001, 
            self.a.y.min(self.b.y).min(self.c.y) - 0.001,
            self.a.z.min(self.b.z).min(self.c.z) - 0.001,
        );
        let max = crate::math::vector3d::Vec3::new(
            self.a.x.max(self.b.x).max(self.c.x) + 0.001,
            self.a.y.max(self.b.y).max(self.c.y) + 0.001,
            self.a.z.max(self.b.z).max(self.c.z) + 0.001,
        );
        AABB { min, max }
    }
}