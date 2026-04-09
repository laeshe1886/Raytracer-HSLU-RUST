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
    let shadow_ray = Ray { origin: point, direction: light_dir };
    for object in &scene.objects {
        if let Some(sh) = object.intersect(&shadow_ray) {
            if let Material::Dielectric { .. } = sh.material {
                    continue; 
                }
                return true;
        }
    }
    false
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(&n))
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Option<Vec3> {
    let cos_theta = (uv * -1.0).dot(&n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let discriminant = 1.0 - r_out_perp.dot(&r_out_perp);
    if discriminant > 0.0 {
        let r_out_parallel = n * -(discriminant.sqrt());
        Some(r_out_perp + r_out_parallel)
    } else {
        None // Totale innere Reflexion
    }
}

// === SHADER ===
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

pub fn shade(hit: &Hit, scene: &Scene, ray: &Ray) -> Vec3 {
    match hit.material {
        Material::Lambert { ambient, albedo } => shade_lambert(hit, scene, ambient, albedo),
        Material::Phong { ambient, albedo, shininess, kd, ka, ks } => shade_phong(hit, scene, ray, ambient, albedo, shininess, kd, ka, ks),
        Material::BlinnPhong { ambient, albedo, shininess, kd, ka, ks } => shade_blinn_phong(hit, scene, ray, ambient, albedo, shininess, kd, ka, ks),
        _ => Vec3::new(0.0, 0.0, 0.0) // Wird in trace_ray separat behandelt
    }
}

// === NEU: DIE REKURSIVE HAUPTFUNKTION ===
pub fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    // Abbruchbedingung für Rekursion (verhindert Endlosschleifen zwischen Spiegeln)
    if depth >= 50 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = intersect_scene(ray, scene) {
        match hit.material {
            Material::Lambert { .. } | Material::Phong { .. } | Material::BlinnPhong { .. } => {
                shade(&hit, scene, ray)
            },
            Material::Metal { specular_color, glossiness: _ } => {
                let reflected_dir = reflect(ray.direction.normalize(), hit.normal);
                let scattered_ray = Ray {
                    origin: hit.point + hit.normal * 0.01,
                    direction: reflected_dir,
                };
                let bounce_color = trace_ray(&scattered_ray, scene, depth + 1);
                
                Vec3::new(
                    specular_color.x * bounce_color.x,
                    specular_color.y * bounce_color.y,
                    specular_color.z * bounce_color.z
                )
            },
            Material::Dielectric { refractive_index, absorption: _ } => {
                let refraction_ratio = if hit.frontface { 1.0 / refractive_index } else { refractive_index };
                let unit_direction = ray.direction.normalize();
                
                let cos_theta = (unit_direction * -1.0).dot(&hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                
                let direction = if cannot_refract {
                    reflect(unit_direction, hit.normal)
                } else {
                    refract(unit_direction, hit.normal, refraction_ratio).unwrap_or_else(|| reflect(unit_direction, hit.normal))
                };
                
                // Wir müssen den Startpunkt korrekt auf die Seite schieben, in die der Strahl weiterfliegt
                let normal_offset = if direction.dot(&hit.normal) > 0.0 { hit.normal } else { hit.normal * -1.0 };
                
                let scattered_ray = Ray {
                    origin: hit.point + normal_offset * 0.01,
                    direction,
                };
                
                trace_ray(&scattered_ray, scene, depth + 1)
            }
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

pub fn calculate_pixel_color(x: usize, y: usize, width: usize, height: usize, scene: &Scene) -> u32 {
    let ray = scene.camera.make_ray(x, y, width, height);
    
    // Start der Rekursion mit depth = 0
    let final_color_vec = trace_ray(&ray, scene, 0);

    let c = Color::new(final_color_vec.x, final_color_vec.y, final_color_vec.z).clamp();
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