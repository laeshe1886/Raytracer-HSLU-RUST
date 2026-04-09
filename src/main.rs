mod math;
mod geometry;
mod render;
mod scene;
mod camera;
mod material;

use minifb::{Key, Window, WindowOptions};
use scene::Scene;
use render::{draw_pixels, RenderData};

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    // Wähle hier die gewünschte Szene aus:
    let scene = Scene::default_scene(WIDTH, HEIGHT);
    //let scene = Scene::vogelperspektive_scene(WIDTH, HEIGHT);
    // let scene = Scene::nahaufnahme_scene(WIDTH, HEIGHT);
    // let scene = Scene::froschperspektive_scene(WIDTH, HEIGHT);
    // let scene = Scene::weitwinkel_scene(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Raytracer Rust - HSLU",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).expect("Cannot open window");

    // Limitiere die Framerate, damit die CPU nicht unnötig heiss läuft
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let data = RenderData {
            height: HEIGHT,
            width: WIDTH,
            buffer: &mut buffer,
            scene: &scene, 
        };

        draw_pixels(data);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}