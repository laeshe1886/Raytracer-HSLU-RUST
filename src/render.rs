use crate::vector3d::Vec3;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::object::Hit;

pub struct RenderData<'a> {
    pub start_x: usize,
    pub end_x: usize,
    pub height: usize,
    pub width: usize,
    pub buffer: &'a mut [u32],
    pub scene: &'a Scene,
}

pub fn calculate_pixel_color(x: usize, y: usize, width: usize, height: usize, scene: &Scene) -> u32 {
    let aspect_ratio = width as f32 / height as f32;
    let px = (2.0 * ((x as f32 + 0.5) / width as f32) - 1.0) * aspect_ratio;
    let py = 1.0 - 2.0 * ((y as f32 + 0.5) / height as f32);

    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(px, py, -1.0).normalize(),
    };

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

    if let Some(hit) = closest_hit {
        let light_dir = (scene.light_pos - hit.point).normalize();
        let light_distance = (scene.light_pos - hit.point).length();
        
        let eps = 0.001;
        let shadow_ray = Ray {
            origin: hit.point + light_dir * eps,
            direction: light_dir,
        };

        let mut in_shadow = false;

        for object in &scene.objects {
            if let Some(shadow_hit) = object.intersect(&shadow_ray) {
                if shadow_hit.distance < light_distance {
                    in_shadow = true;
                    break; 
                }
            }
        }

        let ambient = 0.2;
        let mut diffuse = 0.0;

        if !in_shadow {
            diffuse = light_dir.dot(&hit.normal).max(0.0);
        }
        
        let final_intensity = ambient + diffuse;
        let final_color = hit.color * final_intensity;
        
        let r = (final_color.x.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (final_color.y.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (final_color.z.clamp(0.0, 1.0) * 255.0) as u32;
        
        return (r << 16) | (g << 8) | b;
    }
    
    0x000000 
}

pub fn draw_pixels(data: RenderData) {
    for y in 0..data.height {
        for x in data.start_x..data.end_x {
            let index = y * data.width + x;
            data.buffer[index] = calculate_pixel_color(x, y, data.width, data.height, data.scene);
        }
    }
}