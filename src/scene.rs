use crate::math::vector3d::Vec3;
use crate::math::color::Color;
use crate::geometry::hittable::Hittable;
use crate::geometry::sphere::Sphere;
use crate::geometry::triangle::Triangle;
use crate::geometry::plane::Plane;
use crate::camera::Camera;
use crate::material::Material;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Vec3>,
    pub camera: Camera,
    pub background_color: Color,
}

impl Scene {
    pub fn add_cube_mesh(objects: &mut Vec<Box<dyn Hittable>>, min: Vec3, max: Vec3, material: Material) {
        let v0 = Vec3::new(min.x, min.y, max.z); 
        let v1 = Vec3::new(max.x, min.y, max.z); 
        let v2 = Vec3::new(max.x, max.y, max.z); 
        let v3 = Vec3::new(min.x, max.y, max.z); 
        
        let v4 = Vec3::new(min.x, min.y, min.z); 
        let v5 = Vec3::new(max.x, min.y, min.z); 
        let v6 = Vec3::new(max.x, max.y, min.z); 
        let v7 = Vec3::new(min.x, max.y, min.z); 

        let mut add_face = |a, b, c, d| {
            objects.push(Box::new(Triangle { a, b, c, material }));
            objects.push(Box::new(Triangle { a, b: c, c: d, material }));
        };

        add_face(v0, v1, v2, v3);
        add_face(v5, v4, v7, v6);
        add_face(v1, v5, v6, v2);
        add_face(v4, v0, v3, v7);
        add_face(v3, v2, v6, v7);
        add_face(v4, v5, v1, v0);
    }

    pub fn default_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        
        objects.push(Box::new(Sphere {
            center: Vec3::new(-2.0, 0.0, -6.0), 
            radius: 2.0,
            material: Material::BlinnPhong {
                ambient: 0.1,
                albedo: Vec3::new(1.0, 0.0, 0.0),
                shininess: 50.0,
                kd: 0.8,
                ka: 1.0,
                ks: 0.8,
            },
        }));

        objects.push(Box::new(Triangle {
            a: Vec3::new(0.0, 2.0, -4.0),
            b: Vec3::new(-1.5, 0.0, -4.0),
            c: Vec3::new(1.5, 0.0, -4.0),
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(1.0, 1.0, 0.0),
            },
        }));

        objects.push(Box::new(Plane {
            point: Vec3::new(0.0, -2.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0).normalize(), 
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.2, 0.5, 0.8),
            },
        }));

        Self::add_cube_mesh(
            &mut objects, 
            Vec3::new(1.0, -2.0, -7.0), 
            Vec3::new(4.0, 1.0, -4.0), 
            Material::Phong {
                ambient: 0.1,
                albedo: Vec3::new(0.0, 1.0, 0.0),
                shininess: 30.0,
                kd: 0.7,
                ka: 1.0,
                ks: 0.9,
            }
        );

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(0.0, 0.0, -1.0), 
            Vec3::new(0.0, 1.0, 0.0), 
            90.0, 
            width as f32 / height as f32
        );

        Self {
            objects,
            lights: vec![
                Vec3::new(5.0, 5.0, 0.0),
                Vec3::new(-5.0, 5.0, 0.0), 
            ],
            camera,
            background_color: Color::new(0.1, 0.1, 0.1), 
        }
    }

    pub fn vogelperspektive_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        
        objects.push(Box::new(Plane {
            point: Vec3::new(0.0, -2.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0).normalize(), 
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.3, 0.3, 0.3),
            },
        }));

        for i in 0..3 {
            Self::add_cube_mesh(
                &mut objects, 
                Vec3::new(i as f32 * 3.0 - 3.0, -2.0, -12.0), 
                Vec3::new(i as f32 * 3.0 - 1.5, i as f32 + 1.0, -10.5), 
                Material::Phong {
                    ambient: 0.1,
                    albedo: Vec3::new(0.8, 0.2, 0.2),
                    shininess: 40.0,
                    kd: 0.8,
                    ka: 1.0,
                    ks: 0.7,
                }
            );
        }

        for x in -2..2 {
            objects.push(Box::new(Sphere {
                center: Vec3::new(x as f32 * 4.0, -1.0, -15.0),
                radius: 0.8,
                material: Material::BlinnPhong {
                    ambient: 0.1,
                    albedo: Vec3::new(0.2, 0.7, 0.2),
                    shininess: 80.0,
                    kd: 0.7,
                    ka: 1.0,
                    ks: 0.9,
                },
            }));
        }

        let camera = Camera::new(
            Vec3::new(0.0, 12.0, -5.0), 
            Vec3::new(0.0, 0.0, -11.0), 
            Vec3::new(0.0, 0.0, -1.0), 
            65.0, 
            width as f32 / height as f32
        );

        Self {
            objects,
            lights: vec![Vec3::new(0.0, 15.0, -8.0), Vec3::new(5.0, 10.0, -5.0)],
            camera,
            background_color: Color::new(0.05, 0.05, 0.1), 
        }
    }

    pub fn nahaufnahme_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        
        // Sehr glänzende goldene Kugel
        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -5.0), 
            radius: 0.8,
            material: Material::BlinnPhong {
                ambient: 0.1,
                albedo: Vec3::new(1.0, 0.8, 0.0),
                shininess: 150.0, 
                kd: 0.6,
                ka: 1.0,
                ks: 1.0,
            },
        }));

        // Blaue Kugel
        objects.push(Box::new(Sphere {
            center: Vec3::new(1.2, 0.5, -7.0), 
            radius: 0.4,
            material: Material::Phong {
                ambient: 0.15,
                albedo: Vec3::new(0.0, 0.5, 1.0),
                shininess: 50.0,
                kd: 0.8,
                ka: 1.0,
                ks: 0.8,
            },
        }));

        // Mattes grünes Dreieck
        objects.push(Box::new(Triangle {
            a: Vec3::new(-1.5, -0.5, -6.0),
            b: Vec3::new(-0.5, -0.8, -6.0),
            c: Vec3::new(-1.0, 0.5, -6.0),
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.5, 1.0, 0.5),
            },
        }));

        // Grauer Würfel
        Self::add_cube_mesh(
            &mut objects,
            Vec3::new(-2.0, -1.0, -8.0),
            Vec3::new(-1.0, 0.0, -7.0),
            Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.7, 0.7, 0.7),
            }
        );

        let camera = Camera::new(
            Vec3::new(0.8, 0.5, -2.0), 
            Vec3::new(0.0, 0.0, -5.0), 
            Vec3::new(0.0, 1.0, 0.0), 
            30.0, 
            width as f32 / height as f32
        );

        Self {
            objects,
            lights: vec![Vec3::new(2.0, 3.0, -2.0)],
            camera,
            background_color: Color::new(0.02, 0.02, 0.02), 
        }
    }

    pub fn froschperspektive_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        
        objects.push(Box::new(Plane {
            point: Vec3::new(0.0, -1.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0).normalize(), 
            material: Material::Lambert {
                ambient: 0.15,
                albedo: Vec3::new(0.2, 0.2, 0.2),
            },
        }));

        for i in -2..3 {
            let height_val = ((i as i32).abs() + 2) as f32 * 2.0;
            Self::add_cube_mesh(
                &mut objects, 
                Vec3::new(i as f32 * 3.0 - 0.5, -1.0, -12.0), 
                Vec3::new(i as f32 * 3.0 + 0.5, height_val - 1.0, -11.0), 
                Material::Phong {
                    ambient: 0.1,
                    albedo: Vec3::new(0.2, 0.6, 0.9),
                    shininess: 30.0,
                    kd: 0.8,
                    ka: 1.0,
                    ks: 0.5,
                }
            );
        }

        // Weisse "Sonnen"-Kugel weit oben
        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 10.0, -15.0),
            radius: 3.0,
            material: Material::Lambert {
                ambient: 0.8, // Hoher Ambient-Wert, damit sie selbst leuchtend aussieht
                albedo: Vec3::new(1.0, 1.0, 1.0),
            },
        }));

        let camera = Camera::new(
            Vec3::new(0.0, -0.5, -4.0), 
            Vec3::new(0.0, 5.0, -12.0), 
            Vec3::new(0.0, 1.0, 0.0), 
            85.0, 
            width as f32 / height as f32
        );

        Self {
            objects,
            lights: vec![Vec3::new(10.0, 20.0, -5.0)],
            camera,
            background_color: Color::new(0.5, 0.7, 1.0), 
        }
    }

    pub fn weitwinkel_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        
        objects.push(Box::new(Plane {
            point: Vec3::new(0.0, -2.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0).normalize(), 
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.4, 0.4, 0.4),
            },
        }));

        objects.push(Box::new(Triangle {
            a: Vec3::new(-5.0, -2.0, -5.0),
            b: Vec3::new(-5.0, 5.0, -15.0),
            c: Vec3::new(-5.0, -2.0, -15.0),
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.8, 0.8, 0.8),
            },
        }));

        objects.push(Box::new(Triangle {
            a: Vec3::new(5.0, -2.0, -5.0),
            b: Vec3::new(5.0, 5.0, -15.0),
            c: Vec3::new(5.0, -2.0, -15.0),
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.8, 0.8, 0.8),
            },
        }));

        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -8.0), 
            radius: 1.5,
            material: Material::BlinnPhong {
                ambient: 0.1,
                albedo: Vec3::new(1.0, 0.2, 0.2),
                shininess: 60.0,
                kd: 0.9,
                ka: 1.0,
                ks: 0.9,
            },
        }));

        Self::add_cube_mesh(
            &mut objects,
            Vec3::new(-1.0, -2.0, -5.0),
            Vec3::new(1.0, 0.0, -4.0),
            Material::Phong {
                ambient: 0.1,
                albedo: Vec3::new(0.2, 0.2, 1.0),
                shininess: 40.0,
                kd: 0.7,
                ka: 1.0,
                ks: 0.8,
            }
        );

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, -1.0), 
            Vec3::new(0.0, 0.0, -10.0), 
            Vec3::new(0.0, 1.0, 0.0), 
            110.0, 
            width as f32 / height as f32
        );

        Self {
            objects,
            lights: vec![Vec3::new(0.0, 5.0, -5.0)],
            camera,
            background_color: Color::new(0.1, 0.1, 0.1), 
        }
    }
}