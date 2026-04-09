use crate::scene::Scene;
use crate::math::color::Color;
use crate::geometry::hittable::Hit;
use crate::math::ray::Ray;
use rayon::prelude::*;

pub struct RenderData<'a> {
    pub start_x: usize,
    pub end_x: usize,
    pub height: usize,
    pub width: usize,
    pub buffer: &'a mut [u32],
    pub scene: &'a Scene,
}

pub fn calculate_pixel_color(x: usize, y: usize, width: usize, height: usize, scene: &Scene) -> u32 {
    let ray = scene.camera.make_ray(x, y, width, height);

    let mut closest_hit: Option<Hit> = None;
    let mut min_distance = f32::MAX;

    for object in &scene.objects {
        if let Some(hit) = object.intersect(&ray) {
            if hit.distance < min_distance {
                min_distance = hit.distance;
                closest_hit = Some(hit);
            }
        }
    }

    let final_color = if let Some(hit) = closest_hit {
        let ambient = 0.2;
        let mut total_diffuse = 0.0;
        let light_count = scene.lights.len() as f32;

        for &light_pos in &scene.lights {
            let light_dir = (light_pos - hit.point).normalize();
            let light_distance = (light_pos - hit.point).length();
            
            let shadow_ray = Ray {
                origin: hit.point + light_dir * 0.001,
                direction: light_dir,
            };

            let mut in_shadow = false;
            for object in &scene.objects {
                if let Some(sh) = object.intersect(&shadow_ray) {
                    if sh.distance < light_distance {
                        in_shadow = true;
                        break;
                    }
                }
            }

            if !in_shadow {
                total_diffuse += light_dir.dot(&hit.normal).max(0.0) / light_count;
            }
        }
        
        let intensity = ambient + total_diffuse;
        Color::new(hit.color.x, hit.color.y, hit.color.z) * intensity
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
    let start_x = data.start_x;
    let end_x = data.end_x;

    data.buffer
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            for x in start_x..end_x {
                row[x] = calculate_pixel_color(x, y, width, height, scene);
            }
        });
}