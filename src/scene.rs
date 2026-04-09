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
        
        // 1. Perfekter Spiegel auf der linken Seite
        objects.push(Box::new(Sphere {
            center: Vec3::new(-3.5, 0.0, -5.0), 
            radius: 2.0,
            material: Material::Metal {
                specular_color: Vec3::new(0.9, 0.9, 0.9), // Silber
                glossiness: 0.0,
            },
        }));

        // 2. Kugel aus Glas auf der rechten Seite
        objects.push(Box::new(Sphere {
            center: Vec3::new(3.5, 0.0, -5.0), 
            radius: 2.0,
            material: Material::Dielectric {
                refractive_index: 1.5, // Glas
                absorption: 0.0,
            },
        }));

        // Rote Blinn-Phong Kugel im Hintergrund
        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -8.0), 
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

        // Matter Lambert-Boden
        objects.push(Box::new(Plane {
            point: Vec3::new(0.0, -2.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0).normalize(), 
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.2, 0.5, 0.8),
            },
        }));

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

        // 4 Kugeln mit unterschiedlichen Materialien, um sie von oben zu betrachten
        let materials = vec![
            Material::Metal { specular_color: Vec3::new(0.8, 0.8, 0.8), glossiness: 0.0 }, // Silber
            Material::Dielectric { refractive_index: 1.5, absorption: 0.0 },               // Glas
            Material::BlinnPhong { ambient: 0.1, albedo: Vec3::new(0.2, 0.7, 0.2), shininess: 80.0, kd: 0.7, ka: 1.0, ks: 0.9 }, // Grün
            Material::Metal { specular_color: Vec3::new(1.0, 0.8, 0.2), glossiness: 0.0 }, // Gold
        ];

        for (i, material) in materials.into_iter().enumerate() {
            let x = i as f32 * 3.0 - 4.5;
            objects.push(Box::new(Sphere {
                center: Vec3::new(x, -1.0, -15.0),
                radius: 1.0,
                material,
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
        
        // Grosse, spiegelnde Goldkugel im Zentrum
        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -5.0), 
            radius: 0.8,
            material: Material::Metal {
                specular_color: Vec3::new(1.0, 0.8, 0.2),
                glossiness: 0.0,
            },
        }));

        // Kleine Glaskugel daneben - etwas näher herangeholt
        objects.push(Box::new(Sphere {
            center: Vec3::new(1.0, 0.3, -4.5), 
            radius: 0.4,
            material: Material::Dielectric {
                refractive_index: 1.5,
                absorption: 0.0,
            },
        }));

        // Kamera etwas weiter zurück und FOV auf 40.0 für besseren Fokus
        let camera = Camera::new(
            Vec3::new(1.0, 0.8, -1.5), 
            Vec3::new(0.0, 0.0, -5.0), 
            Vec3::new(0.0, 1.0, 0.0), 
            40.0, 
            width as f32 / height as f32
        );

        Self {
            objects,
            lights: vec![Vec3::new(3.0, 5.0, -1.0)],
            camera,
            // Hintergrund aufgehellt, damit Metall nicht schwarz spiegelt
            background_color: Color::new(0.15, 0.15, 0.2), 
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
            
            // Die mittlere Säule (i=0) machen wir aus spiegelndem Metall!
            let material = if i == 0 {
                Material::Metal { specular_color: Vec3::new(0.9, 0.9, 0.9), glossiness: 0.0 }
            } else {
                Material::Phong {
                    ambient: 0.1,
                    albedo: Vec3::new(0.2, 0.6, 0.9),
                    shininess: 30.0,
                    kd: 0.8,
                    ka: 1.0,
                    ks: 0.5,
                }
            };

            Self::add_cube_mesh(
                &mut objects, 
                Vec3::new(i as f32 * 3.0 - 0.5, -1.0, -12.0), 
                Vec3::new(i as f32 * 3.0 + 0.5, height_val - 1.0, -11.0), 
                material
            );
        }

        // Weisse "Sonnen"-Kugel weit oben
        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 10.0, -15.0),
            radius: 3.0,
            material: Material::Lambert {
                ambient: 0.8, 
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

        // Wände (Triangles) weiter nach aussen verschoben
        objects.push(Box::new(Triangle {
            a: Vec3::new(-8.0, -2.0, -5.0),
            b: Vec3::new(-8.0, 5.0, -15.0),
            c: Vec3::new(-8.0, -2.0, -15.0),
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.8, 0.8, 0.8),
            },
        }));

        objects.push(Box::new(Triangle {
            a: Vec3::new(8.0, -2.0, -5.0),
            b: Vec3::new(8.0, 5.0, -15.0),
            c: Vec3::new(8.0, -2.0, -15.0),
            material: Material::Lambert {
                ambient: 0.2,
                albedo: Vec3::new(0.8, 0.8, 0.8),
            },
        }));

        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -8.0), 
            radius: 1.5,
            material: Material::Dielectric {
                refractive_index: 1.5,
                absorption: 0.0,
            },
        }));

        // FOV auf 90.0 reduziert, um extreme Randverzerrung zu vermeiden
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(0.0, 0.0, -10.0), 
            Vec3::new(0.0, 1.0, 0.0), 
            90.0, 
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