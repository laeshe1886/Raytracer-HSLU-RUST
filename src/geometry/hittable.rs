use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::material::Material;
use crate::geometry::aabb::AABB;

pub struct Hit {
    pub distance: f32,    
    pub point: Vec3,      
    pub normal: Vec3,
    pub uv: (f32, f32),   
    pub material: Material,
    pub frontface: bool,
}

pub trait Hittable: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn bounding_box(&self) -> AABB;
}