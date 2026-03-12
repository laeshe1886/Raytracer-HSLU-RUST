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
    Cube { min: Vec3, max: Vec3, color: Vec3 },
    Triangle { a: Vec3, b: Vec3, c: Vec3, color: Vec3 },
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

            Object::Cube { min, max, color } => {
                let mut tmin = (min.x - ray.origin.x) / ray.direction.x;
                let mut tmax = (max.x - ray.origin.x) / ray.direction.x;
                if tmin > tmax { std::mem::swap(&mut tmin, &mut tmax); }

                let mut tymin = (min.y - ray.origin.y) / ray.direction.y;
                let mut tymax = (max.y - ray.origin.y) / ray.direction.y;
                if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax); }

                if (tmin > tymax) || (tymin > tmax) { return None; }
                
                if tymin > tmin { tmin = tymin; }
                if tymax < tmax { tmax = tymax; }

                let mut tzmin = (min.z - ray.origin.z) / ray.direction.z;
                let mut tzmax = (max.z - ray.origin.z) / ray.direction.z;
                if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }

                if (tmin > tzmax) || (tzmin > tmax) { return None; }
                
                if tzmin > tmin { tmin = tzmin; }

                if tmin > 0.001 {
                    let hit_point = ray.origin + ray.direction * tmin;
                    
                    let normal = if (hit_point.x - min.x).abs() < 0.001 { Vec3::new(-1.0, 0.0, 0.0) }
                    else if (hit_point.x - max.x).abs() < 0.001 { Vec3::new(1.0, 0.0, 0.0) }
                    else if (hit_point.y - min.y).abs() < 0.001 { Vec3::new(0.0, -1.0, 0.0) }
                    else if (hit_point.y - max.y).abs() < 0.001 { Vec3::new(0.0, 1.0, 0.0) }
                    else if (hit_point.z - min.z).abs() < 0.001 { Vec3::new(0.0, 0.0, -1.0) }
                    else { Vec3::new(0.0, 0.0, 1.0) };

                    return Some(Hit { distance: tmin, point: hit_point, normal, color: *color });
                }
                None
            }

            Object::Triangle { a, b, c, color } => {
                let v = *b - *a;
                let w = *c - *a;
                let n = v.cross(&w).normalize();
                
                let denom = ray.direction.dot(&n);
                if denom.abs() < 1e-6 { return None; }
                
                let lambda = (*a - ray.origin).dot(&n) / denom;
                
                if lambda < 0.001 { return None; } 
                
                let hit_point = ray.origin + ray.direction * lambda; 
                
                let edge_ab = *b - *a;
                let vp_a = hit_point - *a;
                if n.dot(&edge_ab.cross(&vp_a)) < 0.0 { return None; }
                
                let edge_bc = *c - *b;
                let vp_b = hit_point - *b;
                if n.dot(&edge_bc.cross(&vp_b)) < 0.0 { return None; }
                
                let edge_ca = *a - *c;
                let vp_c = hit_point - *c;
                if n.dot(&edge_ca.cross(&vp_c)) < 0.0 { return None; }
                
                Some(Hit { distance: lambda, point: hit_point, normal: n, color: *color })
            }
        }
    }
}