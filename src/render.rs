use crate::scene::Scene;
use crate::math::color::Color;
use crate::geometry::hittable::Hit;
use crate::math::ray::Ray;
use crate::math::vector3d::Vec3;
use crate::material::Material;
use rayon::prelude::*;

pub struct RenderData<'a> {
    pub height: usize,
    pub width: usize,
    pub buffer: &'a mut [u32],
    pub scene: &'a Scene,
}

pub fn intersect_scene(ray: &Ray, scene: &Scene) -> Option<Hit> {
    let mut closest_hit: Option<Hit> = None;
    let mut min_distance = f32::MAX;

    for object in &scene.objects {
        if let Some(hit) = object.intersect(ray) {
            if hit.distance < min_distance {
                min_distance = hit.distance;
                closest_hit = Some(hit);
            }
        }
    }
    closest_hit
}

fn in_shadow(point: Vec3, light_dir: Vec3, light_distance: f32, scene: &Scene) -> bool {
    let shadow_ray = Ray {
        origin: point,
        direction: light_dir,
    };
    for object in &scene.objects {
        if let Some(sh) = object.intersect(&shadow_ray) {
            if sh.distance < light_distance {
                return true;
            }
        }
    }
    false
}

pub fn shade_lambert(hit: &Hit, scene: &Scene, ambient: f32, albedo: Vec3) -> Vec3 {
    let mut total_diffuse = Vec3::new(0.0, 0.0, 0.0);
    let ambient_color = albedo * ambient;
    let light_count = scene.lights.len() as f32;

    for &light_pos in &scene.lights {
        let light_dir = (light_pos - hit.point).normalize();
        let light_distance = (light_pos - hit.point).length();
        let offset_point = hit.point + hit.normal * 0.01;

        if !in_shadow(offset_point, light_dir, light_distance, scene) {
            let n_dot_l = hit.normal.dot(&light_dir).max(0.0);
            total_diffuse = total_diffuse + (albedo * n_dot_l) * (1.0 / light_count);
        }
    }
    ambient_color + total_diffuse
}

pub fn shade_phong(hit: &Hit, scene: &Scene, ray: &Ray, ambient: f32, albedo: Vec3, shininess: f32, kd: f32, ka: f32, ks: f32) -> Vec3 {
    let mut total_color = albedo * (ambient * ka);
    let light_color = Vec3::new(1.0, 1.0, 1.0); 
    let light_count = scene.lights.len() as f32;

    for &light_pos in &scene.lights {
        let light_dir = (light_pos - hit.point).normalize();
        let light_distance = (light_pos - hit.point).length();
        let offset_point = hit.point + hit.normal * 0.01;

        if !in_shadow(offset_point, light_dir, light_distance, scene) {
            let n_dot_l = hit.normal.dot(&light_dir).max(0.0);
            let diffuse = albedo * (kd * n_dot_l);
            
            let mut specular = Vec3::new(0.0, 0.0, 0.0);
            if n_dot_l > 0.0 {
                let dir_dot_n = ray.direction.dot(&hit.normal);
                let r = (ray.direction - hit.normal * (2.0 * dir_dot_n)).normalize();
                let r_dot_l = r.dot(&light_dir).max(0.0);
                
                let specular_intensity = ks * r_dot_l.powf(shininess);
                specular = light_color * specular_intensity;
            }

            total_color = total_color + (diffuse + specular) * (1.0 / light_count);
        }
    }
    total_color
}

pub fn shade_blinn_phong(hit: &Hit, scene: &Scene, ray: &Ray, ambient: f32, albedo: Vec3, shininess: f32, kd: f32, ka: f32, ks: f32) -> Vec3 {
    let mut total_color = albedo * (ambient * ka);
    let light_color = Vec3::new(1.0, 1.0, 1.0);
    let v = (ray.direction * -1.0).normalize();
    let light_count = scene.lights.len() as f32;

    for &light_pos in &scene.lights {
        let light_dir = (light_pos - hit.point).normalize();
        let light_distance = (light_pos - hit.point).length();
        let offset_point = hit.point + hit.normal * 0.01;

        if !in_shadow(offset_point, light_dir, light_distance, scene) {
            let n_dot_l = hit.normal.dot(&light_dir).max(0.0);
            let diffuse = albedo * (kd * n_dot_l);
            
            let mut specular = Vec3::new(0.0, 0.0, 0.0);
            if n_dot_l > 0.0 {
                let h = (light_dir + v).normalize();
                let n_dot_h = hit.normal.dot(&h).max(0.0);
                
                let specular_intensity = ks * n_dot_h.powf(shininess);
                specular = light_color * specular_intensity;
            }

            total_color = total_color + (diffuse + specular) * (1.0 / light_count);
        }
    }
    total_color
}

pub fn shade(hit: &Hit, scene: &Scene, ray: &Ray) -> Color {
    let result_vec = match hit.material {
        Material::Lambert { ambient, albedo } => {
            shade_lambert(hit, scene, ambient, albedo)
        },
        Material::Phong { ambient, albedo, shininess, kd, ka, ks } => {
            shade_phong(hit, scene, ray, ambient, albedo, shininess, kd, ka, ks)
        },
        Material::BlinnPhong { ambient, albedo, shininess, kd, ka, ks } => {
            shade_blinn_phong(hit, scene, ray, ambient, albedo, shininess, kd, ka, ks)
        }
    };
    Color::new(result_vec.x, result_vec.y, result_vec.z)
}

pub fn calculate_pixel_color(x: usize, y: usize, width: usize, height: usize, scene: &Scene) -> u32 {
    let ray = scene.camera.make_ray(x, y, width, height);

    let final_color = if let Some(hit) = intersect_scene(&ray, scene) {
        shade(&hit, scene, &ray)
    } else {
        scene.background_color
    };

    let c = final_color.clamp();
    let r = (c.r * 255.0) as u32;
    let g = (c.g * 255.0) as u32;
    let b = (c.b * 255.0) as u32;
    (r << 16) | (g << 8) | b
}

pub fn draw_pixels(data: RenderData) {
    let width = data.width;
    let height = data.height;
    let scene = data.scene;

    data.buffer.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
        for (x, pixel) in row.iter_mut().enumerate() {
            *pixel = calculate_pixel_color(x, y, width, height, scene);
        }
    });
}