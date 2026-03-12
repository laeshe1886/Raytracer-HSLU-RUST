use crate::math::vector3d::Vec3;
use crate::geometry::hittable::Hittable;
use crate::geometry::sphere::Sphere;
use crate::geometry::triangle::Triangle;
use crate::geometry::plane::Plane;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Vec3>,
}

impl Scene {

    pub fn add_cube_mesh(objects: &mut Vec<Box<dyn Hittable>>, min: Vec3, max: Vec3, color: Vec3) {
        let v0 = Vec3::new(min.x, min.y, max.z); 
        let v1 = Vec3::new(max.x, min.y, max.z); 
        let v2 = Vec3::new(max.x, max.y, max.z); 
        let v3 = Vec3::new(min.x, max.y, max.z); 
        
        let v4 = Vec3::new(min.x, min.y, min.z); 
        let v5 = Vec3::new(max.x, min.y, min.z); 
        let v6 = Vec3::new(max.x, max.y, min.z); 
        let v7 = Vec3::new(min.x, max.y, min.z); 

        let mut add_face = |a, b, c, d| {
            objects.push(Box::new(Triangle { a, b, c, color }));
            objects.push(Box::new(Triangle { a: a, b: c, c: d, color }));
        };

        add_face(v0, v1, v2, v3);
        add_face(v5, v4, v7, v6);
        add_face(v1, v5, v6, v2);
        add_face(v4, v0, v3, v7);
        add_face(v3, v2, v6, v7);
        add_face(v4, v5, v1, v0);
    }

    pub fn default_scene() -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        
        objects.push(Box::new(Sphere {
            center: Vec3::new(-2.0, 0.0, -6.0), 
            radius: 2.0,
            color: Vec3::new(1.0, 0.0, 0.0), 
        }));

        objects.push(Box::new(Triangle {
            a: Vec3::new(0.0, 2.0, -4.0),
            b: Vec3::new(-1.5, 0.0, -4.0),
            c: Vec3::new(1.5, 0.0, -4.0),
            color: Vec3::new(1.0, 1.0, 0.0), 
        }));

        objects.push(Box::new(Plane {
            point: Vec3::new(0.0, -2.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0).normalize(), 
            color: Vec3::new(0.2, 0.5, 0.8), 
        }));

        Self::add_cube_mesh(
            &mut objects, 
            Vec3::new(1.0, -2.0, -7.0), 
            Vec3::new(4.0, 1.0, -4.0), 
            Vec3::new(0.0, 1.0, 0.0) 
        );

        Self {
            objects,
            lights: vec![
                Vec3::new(5.0, 5.0, 0.0),
                Vec3::new(-5.0, 5.0, 0.0), 
            ],
        }
    }
}