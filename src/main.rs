mod hud;
mod map;
mod player;
mod raycast;
mod utils;

use crate::map::GameMap;
use crate::player::Player;
use minifb::{Window, WindowOptions};

fn main() {
    let mut window = Window::new("MiniEngine", 800, 600, WindowOptions::default()).unwrap();
    let mut buffer: Vec<u32> = vec![0; 800 * 600];

    let mut player = Player::new(1.5, 1.5, 0.0);
    let map = GameMap::default();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0;
        }

        player.update(&map, &window);
        raycast::render_scene(&player, &map, &mut buffer);
        hud::draw_minimap(&player, &map, &mut buffer);
        window.update_with_buffer(&buffer, 800, 600).unwrap();
    }
}
