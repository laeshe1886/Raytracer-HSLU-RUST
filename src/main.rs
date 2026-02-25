mod vector2d;
use vector2d::Vector2D;
use minifb::{Key, Window, WindowOptions};

struct RenderData<'a> {
    start_x: usize,
    end_x: usize,
    height: usize,
    width: usize,
    buffer: &'a mut [u32],
}

fn calculate_pixel_color(x: usize, y: usize, width: usize, height: usize) -> u32 {
    const R: f32 = 200.0;

    let current_pos = Vector2D::new(
        x as f32 - (width as f32 / 2.0),
        y as f32 - (height as f32 / 2.0)
    );

    if current_pos.length() <= R {
        return 0xFF0000; 
    }
    
    0x000000 
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