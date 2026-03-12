use crate::vector3d::Vec3;
use crate::object::Object;

pub struct Scene {
    pub objects: Vec<Object>,
    pub light_pos: Vec3,
}

impl Scene {
    pub fn default_scene() -> Self {
        Self {
            objects: vec![
                Object::Sphere {
                    center: Vec3::new(0.0, 0.0, -5.0),
                    radius: 2.0,
                    color: Vec3::new(1.0, 0.0, 0.0), 
                },
                Object::Plane {
                    point: Vec3::new(0.0, -2.0, 0.0), 
                    normal: Vec3::new(0.0, 1.0, 0.0).normalize(), 
                    color: Vec3::new(0.2, 0.5, 0.8), 
                },
            ],
            light_pos: Vec3::new(5.0, 5.0, 0.0),
        }
    }
}