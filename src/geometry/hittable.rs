use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;

pub struct Hit {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

pub trait Hittable: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}