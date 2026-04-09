use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut t_min = f32::MIN;
        let mut t_max = f32::MAX;

        // X-Achse
        let inv_dx = 1.0 / ray.direction.x;
        let mut tx0 = (self.min.x - ray.origin.x) * inv_dx;
        let mut tx1 = (self.max.x - ray.origin.x) * inv_dx;
        if inv_dx < 0.0 { std::mem::swap(&mut tx0, &mut tx1); }
        t_min = t_min.max(tx0);
        t_max = t_max.min(tx1);

        // Y-Achse
        let inv_dy = 1.0 / ray.direction.y;
        let mut ty0 = (self.min.y - ray.origin.y) * inv_dy;
        let mut ty1 = (self.max.y - ray.origin.y) * inv_dy;
        if inv_dy < 0.0 { std::mem::swap(&mut ty0, &mut ty1); }
        t_min = t_min.max(ty0);
        t_max = t_max.min(ty1);

        // Z-Achse
        let inv_dz = 1.0 / ray.direction.z;
        let mut tz0 = (self.min.z - ray.origin.z) * inv_dz;
        let mut tz1 = (self.max.z - ray.origin.z) * inv_dz;
        if inv_dz < 0.0 { std::mem::swap(&mut tz0, &mut tz1); }
        t_min = t_min.max(tz0);
        t_max = t_max.min(tz1);

        t_max > t_min && t_max > 0.0
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Vec3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        AABB { min: small, max: big }
    }
}