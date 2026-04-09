use crate::math::vector3d::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    Lambert {
        ambient: f32,
        albedo: Vec3,
    },
    Phong {
        ambient: f32,
        albedo: Vec3,
        shininess: f32,
        kd: f32,
        ka: f32,
        ks: f32,
    },
    BlinnPhong {
        ambient: f32,
        albedo: Vec3,
        shininess: f32,
        kd: f32,
        ka: f32,
        ks: f32,
    },
    Metal {
    specular_color: Vec3,
    glossiness: f32,
    },
    Dielectric {
        refractive_index: f32, 
        absorption: Vec3,
    },
}