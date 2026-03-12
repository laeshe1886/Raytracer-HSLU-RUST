mod vector3d;
use vector3d::Vec3;
use minifb::{Key, Window, WindowOptions};

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

struct Sphere {
    center: Vec3,
    radius: f32,
    color: Vec3, // RGB im Bereich 0.0 bis 1.0
}

struct RenderData<'a> {
    start_x: usize,
    end_x: usize,
    height: usize,
    width: usize,
    buffer: &'a mut [u32],
}

fn calculate_pixel_color(x: usize, y: usize, width: usize, height: usize) -> u32 {
    // 1. Strahl (Ray) für das aktuelle Pixel erzeugen
    // Wir mappen die Pixelkoordinaten in den Bereich [-1, 1]
    let aspect_ratio = width as f32 / height as f32;
    let px = (2.0 * ((x as f32 + 0.5) / width as f32) - 1.0) * aspect_ratio;
    let py = 1.0 - 2.0 * ((y as f32 + 0.5) / height as f32);

    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0), // Augpunkt im Ursprung
        direction: Vec3::new(px, py, -1.0).normalize(), // Blickrichtung in die Tiefe (-z)
    };

    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -5.0), // 5 Einheiten entfernt
        radius: 2.0,
        color: Vec3::new(1.0, 0.0, 0.0), // Rote Kugel
    };

    let light_pos = Vec3::new(5.0, 5.0, 0.0); // Lichtquelle weiter oben rechts

    // 2. Schnittpunktberechnung (Kugel mit Ray schneiden)
    // Entspricht der Formel: v = p - c
    let v = ray.origin - sphere.center; 
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&v);
    let c = v.dot(&v) - sphere.radius * sphere.radius;
    
    // Diskriminante d = b^2 - 4ac
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        // Berechne das kleinere Lambda (q1)
        let lambda = (-b - discriminant.sqrt()) / (2.0 * a);
        
        if lambda > 0.0 {
            // Trefferpunkt Q = p + lambda * u
            let hit_point = ray.origin + ray.direction * lambda;
            
            // Oberflächennormale berechnen
            let normal = (hit_point - sphere.center).normalize();
            
            // Vektor zum Licht berechnen
            let light_dir = (light_pos - hit_point).normalize();
            
            // 3. Schattierung (Diffuse Farbabstrahlung + Ambient)
            let ambient = 0.2; // Umgebungsbeleuchtung
            let diffuse = light_dir.dot(&normal).max(0.0); // cos(delta)
            
            let final_intensity = ambient + diffuse;
            let final_color = sphere.color * final_intensity;
            
            // Konvertiere die Farbe von [0.0, 1.0] zurück in u32 Hex
            let r = (final_color.x.clamp(0.0, 1.0) * 255.0) as u32;
            let g = (final_color.y.clamp(0.0, 1.0) * 255.0) as u32;
            let b = (final_color.z.clamp(0.0, 1.0) * 255.0) as u32;
            
            return (r << 16) | (g << 8) | b;
        }
    }
    
    0x000000 // Hintergrundfarbe (Schwarz)
}

fn draw_pixels(data: RenderData) {
    for y in 0..data.height {
        for x in data.start_x..data.end_x {
            let index = y * data.width + x;
            data.buffer[index] = calculate_pixel_color(x, y, data.width, data.height);
        }
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Raytracer Rust - HSLU",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).expect("Cannot open window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let data = RenderData {
            start_x: 0,
            end_x: WIDTH,
            height: HEIGHT,
            width: WIDTH,
            buffer: &mut buffer,
        };

        draw_pixels(data);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}