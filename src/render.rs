use crate::scene::Scene;
use crate::math::color::Color;
use crate::geometry::hittable::Hit;
use crate::math::ray::Ray;
use crate::math::vector3d::Vec3;
use crate::material::Material;
use rayon::prelude::*;
use rand::Rng;


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

/// Berechnet die Abschwaechung des Lichts auf dem Weg zur Lichtquelle.
/// Beruecksichtigt Absorption bei Dielektrika nach Lambert-Beer.
fn get_light_attenuation(mut origin: Vec3, direction: Vec3, light_distance: f32, scene: &Scene) -> Vec3 {
    let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
    let mut remaining_dist = light_distance;

    // Wir verfolgen den Pfad schrittweise, um alle transparenten Objekte zu finden
    while remaining_dist > 0.001 {
        let shadow_ray = Ray { origin, direction };
        
        if let Some(sh) = intersect_scene(&shadow_ray, scene) {
            // Wenn das naechste Objekt weiter weg ist als die Lichtquelle, sind wir fertig
            if sh.distance >= remaining_dist {
                break;
            }

            match sh.material {
                Material::Dielectric { absorption, .. } => {
                    // Refraktion wird bei Schatten ignoriert
                    if sh.frontface {
                        // Strahl tritt ein -> wir suchen den Austrittspunkt am selben Objekt
                        let exit_ray = Ray { 
                            origin: sh.point + direction * 0.001, 
                            direction 
                        };
                        
                        // Da der Strahl nun im Objekt ist, ist der naechste Treffer der Austrittspunkt
                        if let Some(exit_sh) = intersect_scene(&exit_ray, scene) {
                            let d = exit_sh.distance;
                            
                            // Absorption berechnen: I = I0 * e^(-sigma * lambda)
                            let abs_v = Vec3::new(
                                (-absorption.x * d).exp(),
                                (-absorption.y * d).exp(),
                                (-absorption.z * d).exp(),
                            );
                            
                            attenuation = Vec3::new(
                                attenuation.x * abs_v.x,
                                attenuation.y * abs_v.y,
                                attenuation.z * abs_v.z
                            );
                            
                            // Hinter dem Austrittspunkt weitersuchen
                            origin = exit_sh.point + direction * 0.001;
                            remaining_dist -= sh.distance + d + 0.001;
                        } else {
                            break;
                        }
                    } else {
                        // Rueckseite getroffen (Schattenstrahl startet evtl. im Glas oder trifft zuerst eine Rueckseite)
                        let d = sh.distance;
                        let abs_v = Vec3::new(
                            (-absorption.x * d).exp(),
                            (-absorption.y * d).exp(),
                            (-absorption.z * d).exp(),
                        );
                        attenuation = Vec3::new(
                            attenuation.x * abs_v.x,
                            attenuation.y * abs_v.y,
                            attenuation.z * abs_v.z
                        );
                        origin = sh.point + direction * 0.001;
                        remaining_dist -= sh.distance + 0.001;
                    }
                }
                _ => {
                    // Undurchsichtige Objekte blockieren das Licht weiterhin vollstaendig
                    return Vec3::new(0.0, 0.0, 0.0);
                }
            }
        } else {
            break;
        }
    }
    attenuation
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

        // Schatten-Faktor als Vec3 (gefiltertes Licht)
        let shadow_factor = get_light_attenuation(offset_point, light_dir, light_distance, scene);
        
        let n_dot_l = hit.normal.dot(&light_dir).max(0.0);
        
        // Das Licht wird mit dem shadow_factor (Farbe der Absorption) multipliziert
        let diffuse = Vec3::new(
            (albedo.x * n_dot_l) * shadow_factor.x,
            (albedo.y * n_dot_l) * shadow_factor.y,
            (albedo.z * n_dot_l) * shadow_factor.z
        );
        
        total_diffuse = total_diffuse + diffuse * (1.0 / light_count);
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

        let shadow_factor = get_light_attenuation(offset_point, light_dir, light_distance, scene);
        let n_dot_l = hit.normal.dot(&light_dir).max(0.0);
        
        let diffuse = Vec3::new(
            (albedo.x * kd * n_dot_l) * shadow_factor.x,
            (albedo.y * kd * n_dot_l) * shadow_factor.y,
            (albedo.z * kd * n_dot_l) * shadow_factor.z
        );
            
        let mut specular = Vec3::new(0.0, 0.0, 0.0);
        if n_dot_l > 0.0 {
            let dir_dot_n = ray.direction.dot(&hit.normal);
            let r = (ray.direction - hit.normal * (2.0 * dir_dot_n)).normalize();
            let r_dot_l = r.dot(&light_dir).max(0.0);
            let specular_intensity = ks * r_dot_l.powf(shininess);
            specular = Vec3::new(
                (light_color.x * specular_intensity) * shadow_factor.x,
                (light_color.y * specular_intensity) * shadow_factor.y,
                (light_color.z * specular_intensity) * shadow_factor.z
            );
        }
        total_color = total_color + (diffuse + specular) * (1.0 / light_count);
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

        let shadow_factor = get_light_attenuation(offset_point, light_dir, light_distance, scene);
        let n_dot_l = hit.normal.dot(&light_dir).max(0.0);
        
        let diffuse = Vec3::new(
            (albedo.x * kd * n_dot_l) * shadow_factor.x,
            (albedo.y * kd * n_dot_l) * shadow_factor.y,
            (albedo.z * kd * n_dot_l) * shadow_factor.z
        );
            
        let mut specular = Vec3::new(0.0, 0.0, 0.0);
        if n_dot_l > 0.0 {
            let h = (light_dir + v).normalize();
            let n_dot_h = hit.normal.dot(&h).max(0.0);
            let specular_intensity = ks * n_dot_h.powf(shininess);
            specular = Vec3::new(
                (light_color.x * specular_intensity) * shadow_factor.x,
                (light_color.y * specular_intensity) * shadow_factor.y,
                (light_color.z * specular_intensity) * shadow_factor.z
            );
        }
        total_color = total_color + (diffuse + specular) * (1.0 / light_count);
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

pub fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    // Abbruchbedingung fuer Rekursion (verhindert Endlosschleifen zwischen Spiegeln)
    if depth >= 10 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = intersect_scene(ray, scene) {
        match hit.material {
            // --- Matte und glaenzende Standard-Materialien ---
            Material::Lambert { .. } | Material::Phong { .. } | Material::BlinnPhong { .. } => {
                shade(&hit, scene, ray)
            },
            
            // --- Glossy Reflections fuer Metall ---
            Material::Metal { specular_color, glossiness } => {
                let reflected_dir = reflect(ray.direction.normalize(), hit.normal);
                
                // Zufälliger Vektor für matten Glanz
                let mut rng = rand::thread_rng();
                let random_vec = Vec3::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0)
                ).normalize();
                
                let scattered_ray = Ray {
                    // Strahl prallt ab -> Offset auf der Seite der Normalen (+)
                    origin: hit.point + hit.normal * 0.001,
                    direction: (reflected_dir + random_vec * glossiness).normalize(),
                };
                let bounce_color = trace_ray(&scattered_ray, scene, depth + 1);
                
                Vec3::new(
                    specular_color.x * bounce_color.x,
                    specular_color.y * bounce_color.y,
                    specular_color.z * bounce_color.z
                )
            },

            // --- Fresnel, Absorption und Brechung fuer Glas ---
            Material::Dielectric { refractive_index, absorption } => {
                // eta1: Medium aus dem der Strahl kommt, eta2: Medium in das er geht
                let (eta1, eta2) = if hit.frontface {
                    (1.0, refractive_index) // Strahl tritt in das Objekt ein (Luft -> Glas)
                } else {
                    (refractive_index, 1.0) // Strahl tritt aus dem Objekt aus (Glas -> Luft)
                };
                
                let refraction_ratio = eta1 / eta2;
                let unit_direction = ray.direction.normalize();
                
                // hit.normal zeigt in deiner Struktur IMMER entgegen der ray.direction
                let cos_theta = (unit_direction * -1.0).dot(&hit.normal).min(1.0);
                
                // 1. Reflexion (bleibt im selben Medium -> + hit.normal Offset)
                let reflected_dir = reflect(unit_direction, hit.normal);
                let reflected_ray = Ray { 
                    origin: hit.point + hit.normal * 0.001, 
                    direction: reflected_dir 
                };
                let reflected_color = trace_ray(&reflected_ray, scene, depth + 1);
                
                // 2. Brechung (wechselt in das andere Medium -> - hit.normal Offset)
                match refract(unit_direction, hit.normal, refraction_ratio) {
                    Some(refracted_dir) => {
                        let refracted_ray = Ray { 
                            origin: hit.point - hit.normal * 0.001, 
                            direction: refracted_dir 
                        };
                        let mut refracted_color = trace_ray(&refracted_ray, scene, depth + 1);
                        
                        // Absorption nach Lambert-Beer (nur beim Austritt)
                        if !hit.frontface {
                            let absorb_x = (-absorption.x * hit.distance).exp();
                            let absorb_y = (-absorption.y * hit.distance).exp();
                            let absorb_z = (-absorption.z * hit.distance).exp();
                            refracted_color = Vec3::new(
                                refracted_color.x * absorb_x,
                                refracted_color.y * absorb_y,
                                refracted_color.z * absorb_z
                            );
                        }
                        
                        // Schlick-Approximation (Mischt Reflexion und Brechung)
                        let r0 = ((eta1 - eta2) / (eta1 + eta2)).powi(2);
                        let f = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
                        
                        reflected_color * f + refracted_color * (1.0 - f)
                    },
                    None => {
                        // Totale interne Reflexion (Strahl kommt nicht heraus)
                        reflected_color
                    }
                }
            }
        }
    } else {
        // Hintergrundfarbe
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

pub fn calculate_pixel_color(x: usize, y: usize, width: usize, height: usize, scene: &Scene) -> u32 {
    let ray = scene.camera.make_ray(x, y, width, height);
    
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