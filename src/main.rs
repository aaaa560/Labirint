use std::time::{Duration, Instant};

mod hud;
mod map;
mod player;
mod raycast;
mod utils;

use crate::player::Player;
use crate::{hud::draw_hud, map::GameMap};
use minifb::{Window, WindowOptions};

fn main() {
    let mut window = Window::new("MiniEngine", 800, 600, WindowOptions::default()).unwrap();
    let mut buffer: Vec<u32> = vec![0; 800 * 600];

    let mut player = Player::new(1.5, 1.5, 0.0);
    let map = GameMap::default();

    let mut fps_counter = 0;
    let mut fps = 0;
    let mut last_fps_update = Instant::now();
    let fps_update_interval = Duration::from_secs(1);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        fps_counter += 1;
        if last_fps_update.elapsed() >= fps_update_interval {
            fps = fps_counter;
            fps_counter = 0;
            last_fps_update = Instant::now();
            println!("FPS: {}", fps);
        }
        for i in buffer.iter_mut() {
            *i = 0;
        }

        player.update(&map, &window);
        raycast::render_scene(&player, &map, &mut buffer);
        hud::draw_minimap(&player, &map, &mut buffer);
        draw_hud(&player, fps, &mut buffer);
        window.update_with_buffer(&buffer, 800, 600).unwrap();
    }
}
