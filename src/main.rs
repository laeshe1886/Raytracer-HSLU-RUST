mod math;
mod geometry;
mod render;
mod scene;

use minifb::{Key, Window, WindowOptions};
use scene::Scene;
use render::{draw_pixels, RenderData};

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let scene = Scene::default_scene(); 

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
            scene: &scene, 
        };

        draw_pixels(data);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}