mod vector2d;
use vector2d::Vector2D;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Raytracer Rust - SW01",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).expect("Konnte Fenster nicht öffnen");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = y * WIDTH + x;
                
                let r_radius = 200.0;
                let current_pos = Vector2D::new(
                    x as f32 - (WIDTH as f32 / 2.0),
                    y as f32 - (HEIGHT as f32 / 2.0)
                );

                if current_pos.length() <= r_radius {
                    buffer[index] = 0xFF0000; 
                } else {
                    buffer[index] = 0x000000; z
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}