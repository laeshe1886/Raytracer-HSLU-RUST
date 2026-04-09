mod math;
mod geometry;
mod render;
mod scene;
mod camera;
mod material;

use minifb::{Key, Window, WindowOptions};
use scene::Scene;
use render::{draw_pixels, RenderData};
use crate::math::vector3d::Vec3;
use std::env; 
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Hint: You can use arguments to change the view.");
        println!("Usage: cargo run --release -- <scene> <mode>");
        println!("Scenes: default, birds_eye, close_up, frogs_eye, wide_angle, mesh");
        println!("Modes: static, anim\n");
        println!("Starting with default static view...");
    }

    let scene_type = args.get(1).map(|s| s.as_str()).unwrap_or("default");
    let mode = args.get(2).map(|s| s.as_str()).unwrap_or("static");

    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    println!("Initializing Scene: {}...", scene_type);
    let mut scene = match scene_type {
        "mesh" => Scene::mesh_scene(WIDTH, HEIGHT),
        "birds_eye" => Scene::birds_eye_scene(WIDTH, HEIGHT),
        "close_up" => Scene::close_up_scene(WIDTH, HEIGHT),
        "frogs_eye" => Scene::frogs_eye_scene(WIDTH, HEIGHT),
        "wide_angle" => Scene::wide_angle_scene(WIDTH, HEIGHT),
        _ => Scene::default_scene(WIDTH, HEIGHT),
    };

    match scene_type {
        "mesh" => scene.set_camera_orbit(0.0, 12.0, 3.5, Vec3::new(0.0, 1.5, 0.0), 35.0, WIDTH, HEIGHT),
        "birds_eye" => scene.set_camera_orbit(0.0, 9.0, 15.0, Vec3::new(0.0, 0.0, -11.0), 60.0, WIDTH, HEIGHT),
        "close_up" => scene.set_camera_orbit(0.0, 2.8, 1.0, Vec3::new(0.0, 0.0, 0.0), 30.0, WIDTH, HEIGHT),
        "frogs_eye" => scene.set_camera_orbit(0.0, 7.0, -0.6, Vec3::new(0.0, 4.0, -15.0), 85.0, WIDTH, HEIGHT),
        "wide_angle" => scene.set_camera_orbit(0.0, 12.0, 1.0, Vec3::new(0.0, 0.0, -10.0), 110.0, WIDTH, HEIGHT),
        _ => scene.set_camera_orbit(0.0, 12.0, 2.0, Vec3::new(0.0, 0.0, -5.0), 80.0, WIDTH, HEIGHT),
    }

    let mut window = Window::new(
        &format!("Raytracer - Scene: {} | Mode: {}", scene_type, mode),
        WIDTH, HEIGHT, WindowOptions::default(),
    ).expect("Could not open window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if mode == "anim" {
            let elapsed = start_time.elapsed().as_secs_f32() * 0.4;

            match scene_type {
                "mesh" => scene.set_camera_orbit(elapsed, 12.0, 3.5, Vec3::new(0.0, 1.5, 0.0), 35.0, WIDTH, HEIGHT),
                "birds_eye" => scene.set_camera_orbit(elapsed, 9.0, 15.0, Vec3::new(0.0, 0.0, -11.0), 60.0, WIDTH, HEIGHT),
                "close_up" => scene.set_camera_orbit(elapsed, 2.8, 1.0, Vec3::new(0.0, 0.0, 0.0), 30.0, WIDTH, HEIGHT),
                "frogs_eye" => scene.set_camera_orbit(elapsed, 7.0, -0.6, Vec3::new(0.0, 4.0, -15.0), 85.0, WIDTH, HEIGHT),
                "wide_angle" => scene.set_camera_orbit(elapsed, 12.0, 1.0, Vec3::new(0.0, 0.0, -10.0), 110.0, WIDTH, HEIGHT),
                _ => scene.set_camera_orbit(elapsed, 12.0, 2.0, Vec3::new(0.0, 0.0, -5.0), 80.0, WIDTH, HEIGHT),
            }
        }

        draw_pixels(RenderData { height: HEIGHT, width: WIDTH, buffer: &mut buffer, scene: &scene });
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        
        if mode == "static" {
            while window.is_open() && !window.is_key_down(Key::Escape) { window.update(); }
        }
    }
}