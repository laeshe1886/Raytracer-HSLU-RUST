use crate::math::vector3d::Vec3;
use crate::math::color::Color;
use crate::geometry::hittable::Hittable;
use crate::geometry::sphere::Sphere;
use crate::geometry::triangle::Triangle;
use crate::geometry::plane::Plane;
use crate::camera::Camera;
use crate::material::Material;
use crate::geometry::mesh::Mesh;

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
            objects.push(Box::new(Triangle { a, b, c, na: None, nb: None, nc: None, material: material.clone() }));
            objects.push(Box::new(Triangle { a, b: c, c: d, na: None, nb: None, nc: None, material: material.clone() }));
        };

        add_face(v0, v1, v2, v3);
        add_face(v5, v4, v7, v6);
        add_face(v1, v5, v6, v2);
        add_face(v4, v0, v3, v7);
        add_face(v3, v2, v6, v7);
        add_face(v4, v5, v1, v0);
    }

    pub fn set_camera_orbit(&mut self, time: f32, radius: f32, height: f32, target: Vec3, fov: f32, width: usize, height_px: usize) {
        let cam_x = time.sin() * radius + target.x;
        let cam_z = time.cos() * radius + target.z;
        let up = if height > 10.0 { Vec3::new(0.0, 0.0, -1.0) } else { Vec3::new(0.0, 1.0, 0.0) };

        self.camera = Camera::new(
            Vec3::new(cam_x, height, cam_z),
            target,
            up,
            fov,
            width as f32 / height_px as f32
        );
    }

    pub fn monkey_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        
        objects.push(Box::new(Plane {
            point: Vec3::new(0.0, -1.5, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: Material::Metal { 
                specular_color: Vec3::new(0.1, 0.1, 0.15), 
                glossiness: 0.01 
            },
        }));

        let fallback = Material::Lambert { ambient: 0.1, albedo: Vec3::new(0.8, 0.8, 0.8) };
        objects.push(Box::new(Mesh::from_obj("assets/monkey.obj", fallback)));

        objects.push(Box::new(Sphere {
            center: Vec3::new(-3.5, 0.0, 0.0),
            radius: 1.5,
            material: Material::Metal { 
                specular_color: Vec3::new(0.9, 0.9, 1.0), 
                glossiness: 0.0 
            },
        }));

        objects.push(Box::new(Sphere {
            center: Vec3::new(3.5, 0.0, 0.0),
            radius: 1.5,
            material: Material::Dielectric { 
                refractive_index: 1.5, 
                absorption: Vec3::new(0.0, 0.0, 0.0) 
            },
        }));

        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, -0.7, 4.0),
            radius: 0.8,
            material: Material::BlinnPhong { 
                ambient: 0.05, 
                albedo: Vec3::new(0.9, 0.1, 0.1), 
                shininess: 100.0, 
                kd: 0.8, ka: 1.0, ks: 1.0 
            },
        }));

        objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 2.5, -4.0),
            radius: 1.0,
            material: Material::Lambert { 
                ambient: 0.1, 
                albedo: Vec3::new(0.5, 0.5, 0.5) 
            },
        }));

        let lights = vec![
            Vec3::new(10.0, 15.0, 10.0),  
            Vec3::new(-10.0, 10.0, -5.0), 
        ];

        let camera = Camera::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 45.0, width as f32 / height as f32);
        
        Self { 
            objects, 
            lights, 
            camera, 
            background_color: Color::new(0.01, 0.01, 0.03) 
        }
    }

    pub fn mesh_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Plane {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: Material::Metal { specular_color: Vec3::new(0.2, 0.2, 0.2), glossiness: 0.02 },
        }));

        let fallback = Material::Dielectric { refractive_index: 1.5, absorption: Vec3::new(0.0, 0.0, 0.0) };
        objects.push(Box::new(Mesh::from_obj("assets/Glas.obj", fallback)));

        objects.push(Box::new(Sphere {
            center: Vec3::new(5.0, 1.5, -2.0),
            radius: 1.5,
            material: Material::Metal { specular_color: Vec3::new(1.0, 0.8, 0.2), glossiness: 0.0 },
        }));

        let camera = Camera::new(Vec3::new(0.0, 3.0, -12.0), Vec3::new(0.0, 1.5, 0.0), Vec3::new(0.0, 1.0, 0.0), 35.0, width as f32 / height as f32);
        Self { objects, lights: vec![Vec3::new(-15.0, 20.0, 10.0), Vec3::new(15.0, 15.0, -10.0)], camera, background_color: Color::new(0.02, 0.02, 0.05) }
    }

    pub fn birds_eye_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Plane { point: Vec3::new(0.0, -2.0, 0.0), normal: Vec3::new(0.0, 1.0, 0.0), material: Material::Lambert { ambient: 0.1, albedo: Vec3::new(0.1, 0.1, 0.1) } }));

        for i in 0..5 {
            let x = (i as f32 * 3.0) - 6.0;
            let mat = if i % 2 == 0 { Material::Metal { specular_color: Vec3::new(0.8, 0.8, 0.9), glossiness: 0.0 } } 
                      else { Material::Dielectric { refractive_index: 1.5, absorption: Vec3::new(0.0, 0.0, 0.0) } };
            Self::add_cube_mesh(&mut objects, Vec3::new(x, -2.0, -12.0), Vec3::new(x + 1.2, i as f32, -10.0), mat);
        }

        let camera = Camera::new(Vec3::new(0.0, 15.0, -5.0), Vec3::new(0.0, 0.0, -11.0), Vec3::new(0.0, 0.0, -1.0), 60.0, width as f32 / height as f32);
        Self { objects, lights: vec![Vec3::new(0.0, 25.0, -10.0), Vec3::new(10.0, 10.0, 0.0)], camera, background_color: Color::new(0.0, 0.0, 0.02) }
    }

    pub fn close_up_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Sphere { center: Vec3::new(0.0, 0.0, 0.0), radius: 1.0, material: Material::Dielectric { refractive_index: 1.5, absorption: Vec3::new(0.2, 0.05, 0.05) } }));
        objects.push(Box::new(Sphere { center: Vec3::new(1.2, -0.5, -1.0), radius: 0.4, material: Material::Metal { specular_color: Vec3::new(0.9, 0.9, 1.0), glossiness: 0.0 } }));

        let camera = Camera::new(Vec3::new(2.0, 1.0, -3.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 30.0, width as f32 / height as f32);
        Self { objects, lights: vec![Vec3::new(5.0, 5.0, -5.0), Vec3::new(-5.0, 2.0, 2.0)], camera, background_color: Color::new(0.05, 0.05, 0.08) }
    }

    pub fn frogs_eye_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Plane { point: Vec3::new(0.0, -1.0, 0.0), normal: Vec3::new(0.0, 1.0, 0.0), material: Material::Lambert { ambient: 0.1, albedo: Vec3::new(0.2, 0.2, 0.2) } }));
        for i in -3..4 {
            let mat = Material::BlinnPhong { ambient: 0.05, albedo: Vec3::new(0.1, 0.2, 0.5), shininess: 80.0, kd: 0.8, ka: 1.0, ks: 0.9 };
            Self::add_cube_mesh(&mut objects, Vec3::new(i as f32 * 5.0 - 0.5, -1.0, -20.0), Vec3::new(i as f32 * 5.0 + 0.5, 12.0, -19.0), mat);
        }

        let camera = Camera::new(Vec3::new(0.0, -0.7, -6.0), Vec3::new(0.0, 4.0, -15.0), Vec3::new(0.0, 1.0, 0.0), 85.0, width as f32 / height as f32);
        Self { objects, lights: vec![Vec3::new(0.0, 20.0, -12.0)], camera, background_color: Color::new(0.01, 0.01, 0.03) }
    }

    pub fn wide_angle_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Plane { point: Vec3::new(0.0, -2.0, 0.0), normal: Vec3::new(0.0, 1.0, 0.0), material: Material::Lambert { ambient: 0.1, albedo: Vec3::new(0.15, 0.15, 0.15) } }));
        for z in 0..12 {
            let color = if z % 2 == 0 { Vec3::new(0.9, 0.2, 0.2) } else { Vec3::new(0.2, 0.4, 0.9) };
            objects.push(Box::new(Sphere { center: Vec3::new(-5.0, 0.0, z as f32 * -5.0), radius: 1.2, material: Material::Metal { specular_color: color, glossiness: 0.0 } }));
            objects.push(Box::new(Sphere { center: Vec3::new(5.0, 0.0, z as f32 * -5.0), radius: 1.2, material: Material::Metal { specular_color: color, glossiness: 0.0 } }));
        }

        let camera = Camera::new(Vec3::new(0.0, 0.5, 8.0), Vec3::new(0.0, 0.0, -20.0), Vec3::new(0.0, 1.0, 0.0), 115.0, width as f32 / height as f32);
        Self { objects, lights: vec![Vec3::new(0.0, 15.0, 0.0)], camera, background_color: Color::new(0.0, 0.0, 0.0) }
    }

    pub fn default_scene(width: usize, height: usize) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Sphere { center: Vec3::new(-3.0, 0.0, -5.0), radius: 2.0, material: Material::Metal { specular_color: Vec3::new(0.9, 0.9, 0.9), glossiness: 0.02 } }));
        objects.push(Box::new(Sphere { center: Vec3::new(3.0, 0.0, -5.0), radius: 2.0, material: Material::Dielectric { refractive_index: 1.5, absorption: Vec3::new(0.0, 0.0, 0.0) } }));
        objects.push(Box::new(Sphere { center: Vec3::new(0.0, 0.5, -9.0), radius: 2.5, material: Material::BlinnPhong { ambient: 0.1, albedo: Vec3::new(0.9, 0.1, 0.1), shininess: 120.0, kd: 0.8, ka: 1.0, ks: 1.0 } }));
        objects.push(Box::new(Plane { point: Vec3::new(0.0, -2.0, 0.0), normal: Vec3::new(0.0, 1.0, 0.0), material: Material::Lambert { ambient: 0.2, albedo: Vec3::new(0.4, 0.4, 0.4) } }));

        let camera = Camera::new(Vec3::new(0.0, 1.0, 2.0), Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 1.0, 0.0), 80.0, width as f32 / height as f32);
        Self { objects, lights: vec![Vec3::new(10.0, 10.0, 5.0), Vec3::new(-10.0, 10.0, 5.0)], camera, background_color: Color::new(0.08, 0.08, 0.1) }
    }
}