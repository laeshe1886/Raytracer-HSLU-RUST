use crate::vector3d::Vec3;
use crate::object::Object;
use rand::Rng; 

pub struct Scene {
    pub objects: Vec<Object>,
    pub light_pos: Vec3,
}

impl Scene {
    pub fn default_scene() -> Self {
        Self {
            objects: vec![
                Object::Sphere {
                    center: Vec3::new(-2.0, 0.0, -6.0), 
                    radius: 2.0,
                    color: Vec3::new(1.0, 0.0, 0.0),
                },
                Object::Cube {
                    min: Vec3::new(1.0, -2.0, -7.0),  
                    max: Vec3::new(4.0, 1.0, -4.0),   
                    color: Vec3::new(0.0, 1.0, 0.0),
                },
                Object::Plane {
                    point: Vec3::new(0.0, -2.0, 0.0), 
                    normal: Vec3::new(0.0, 1.0, 0.0).normalize(), 
                    color: Vec3::new(0.2, 0.5, 0.8),
                },
                Object::Triangle {
                    a: Vec3::new(0.0, 2.0, -4.0),
                    b: Vec3::new(-1.5, 0.0, -4.0),
                    c: Vec3::new(1.5, 0.0, -4.0),
                    color: Vec3::new(1.0, 1.0, 0.0), 
                },
            ],
            light_pos: Vec3::new(5.0, 5.0, 0.0),
        }
    }

    pub fn random_scene() -> Self {
        let mut objects = Vec::new();
        let mut rng = rand::thread_rng();

        objects.push(Object::Plane {
            point: Vec3::new(0.0, -1.0, 0.0), 
            normal: Vec3::new(0.0, 1.0, 0.0),
            color: Vec3::new(0.5, 0.5, 0.5), 
        });

        for a in -11..11 {
            for b in -20..-2 { 
                
                let center = Vec3::new(
                    a as f32 + 0.9 * rng.r#gen::<f32>(),
                    -0.8, 
                    b as f32 + 0.9 * rng.r#gen::<f32>(),
                );

                let color = Vec3::new(
                    rng.r#gen::<f32>(),
                    rng.r#gen::<f32>(),
                    rng.r#gen::<f32>(),
                );

                objects.push(Object::Sphere {
                    center,
                    radius: 0.2,
                    color,
                });
            }
        }

        objects.push(Object::Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            color: Vec3::new(1.0, 0.2, 0.2), 
        });
        
        objects.push(Object::Sphere {
            center: Vec3::new(-4.0, 0.0, -8.0),
            radius: 1.0,
            color: Vec3::new(0.2, 1.0, 0.2), 
        });

        objects.push(Object::Sphere {
            center: Vec3::new(4.0, 0.0, -6.0),
            radius: 1.0,
            color: Vec3::new(0.2, 0.2, 1.0), 
        });

        Self {
            objects,
            light_pos: Vec3::new(10.0, 20.0, 10.0), 
        }
    }
}