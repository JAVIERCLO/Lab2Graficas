use raylib::prelude::*;
mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const SCALE: i32 = 5;

fn is_alive(color: Color) -> bool {
    color.r > 127 || color.g > 127 || color.b > 127
}

fn count_neighbors(fb: &Framebuffer, x: u32, y: u32) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x.wrapping_add(dx as u32);
            let ny = y.wrapping_add(dy as u32);
            if nx < WIDTH && ny < HEIGHT && is_alive(fb.get_color(nx, ny)) {
                count += 1;
            }
        }
    }
    count
}

fn step(fb: &Framebuffer) -> Vec<(u32, u32, bool)> {
    let mut updates = vec![];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = is_alive(fb.get_color(x, y));
            let neighbors = count_neighbors(fb, x, y);
            let next = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
            if alive != next {
                updates.push((x, y, next));
            }
        }
    }
    updates
}

fn draw(fb: &mut Framebuffer) {
    for (x, y, state) in step(&fb.clone()) {
        if state {
            fb.set_current_color(Color::WHITE);
        } else {
            fb.set_current_color(Color::BLACK);
        }
        fb.set_pixel(x, y);
    }
}

// ====== Organismos ======
fn spawn_glider(fb: &mut Framebuffer, ox: u32, oy: u32) {
    let points = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    fb.set_current_color(Color::WHITE);
    for (dx, dy) in points {
        fb.set_pixel(ox + dx, oy + dy);
    }
}

// Puedes agregar más organismos como pulsar, LWSS aquí

fn main() {
    let (mut rl, thread) = raylib::init()
        .size((WIDTH * SCALE as u32) as i32, (HEIGHT * SCALE as u32) as i32)
        .title("Conway's Game of Life")
        .build();

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);

    // Patrón inicial: varios gliders
    spawn_glider(&mut fb, 10, 10);
    spawn_glider(&mut fb, 30, 20);
    spawn_glider(&mut fb, 50, 50);
    spawn_glider(&mut fb, 70, 70);

    rl.set_target_fps(10);
    while !rl.window_should_close() {
        draw(&mut fb);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let img = fb.get_image();
        let tex = d.load_texture_from_image(&thread, &img).unwrap(); // ✅ FIX aquí
        d.draw_texture_ex(
            &tex,
            Vector2::new(0.0, 0.0),
            0.0,
            SCALE as f32,
            Color::WHITE,
        );
    }
}

