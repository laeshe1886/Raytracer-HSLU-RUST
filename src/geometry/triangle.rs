use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::geometry::hittable::{Hit, Hittable};

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub color: Vec3,
}

impl Hittable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let edge_ab = self.b - self.a;
        let edge_ac = self.c - self.a;
        
        let cross_product = edge_ab.cross(&edge_ac);
        
        let area_abc = cross_product.length() * 0.5;
        
        let n = cross_product.normalize();
        
        let denom = ray.direction.dot(&n);
        if denom.abs() < 1e-6 { return None; }
        
        let lambda = (self.a - ray.origin).dot(&n) / denom;
        if lambda < 0.001 { return None; } 
        
        let p = ray.origin + ray.direction * lambda;
        
        let area_pbc = (self.b - p).cross(&(self.c - p)).length() * 0.5;
        let area_pca = (self.c - p).cross(&(self.a - p)).length() * 0.5;
        let area_pab = (self.a - p).cross(&(self.b - p)).length() * 0.5;
        
        let lambda_a = area_pbc / area_abc;
        let lambda_b = area_pca / area_abc;
        let lambda_c = area_pab / area_abc;
        
        let sum = lambda_a + lambda_b + lambda_c;
        
        if sum > 1.001 {
            return None; 
        }
        
        Some(Hit {
            distance: lambda,
            point: p,
            normal: n,
            color: self.color,
        })
    }
}