use crate::map::GameMap;
use crate::player::Player;

const WIDTH: usize = 800;
const HEIGTH: usize = 600;

pub fn draw_minimap(player: &Player, map: &GameMap, buffer: &mut [u32]) {
    let scale = 6;
    let minimap_size = 150;
    let offset_x = WIDTH - minimap_size - 10;
    let offset_y = 10;

    for y in 0..minimap_size {
        for x in 0..minimap_size {
            let px = offset_x + x;
            let py = offset_y + y;
            if px < WIDTH && py < HEIGTH {
                let current_color = buffer[py * WIDTH + px];
                let r = ((current_color >> 16) & 0xFF) / 2;
                let g = ((current_color >> 8) & 0xFF) / 2;
                let b = (current_color & 0xFF) / 2;
                buffer[py * WIDTH + px] = (r << 16) | (g << 8) | b;
            }
        }
    }

    let view_x = (player.x * scale as f64) as isize - (minimap_size / 2) as isize;
    let view_y = (player.y * scale as f64) as isize - (minimap_size / 2) as isize;

    for y in 0..minimap_size {
        for x in 0..minimap_size {
            let map_x = (view_x + x as isize) / scale as isize;
            let map_y = (view_y + y as isize) / scale as isize;

            if map_x >= 0 && map_x < map.width as isize && map_y >= 0 && map_y < map.height as isize
            {
                let cell_value = map.grid[map_y as usize][map_x as usize];

                if cell_value > 0 {
                    let px = offset_x + x;
                    let py = offset_y + y;

                    if px < WIDTH && py < HEIGTH {
                        let color = match cell_value {
                            1 => 0x8888FF,
                            _ => 0x8888FF,
                        };
                        buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }
    }

    let player_minimap_x = offset_x + (minimap_size / 2);
    let player_minimap_y = offset_y + (minimap_size / 2);

    let player_radius = 3;
    for dy in -player_radius..=player_radius {
        for dx in -player_radius..=player_radius {
            let px = player_minimap_x as isize + dx;
            let py = player_minimap_y as isize + dy;

            if px >= 0 && px < WIDTH as isize && py >= 0 && py < HEIGTH as isize {
                buffer[py as usize * WIDTH + px as usize] = 0xFF0000;
            }
        }
    }

    let direction_lenght = 10;
    let end_x = player_minimap_x as f64 + player.angle.to_radians().cos() * direction_lenght as f64;
    let end_y = player_minimap_y as f64 + player.angle.to_radians().sin() * direction_lenght as f64;

    let steps = direction_lenght * 2;
    for i in 0..steps {
        let t = i as f64 / steps as f64;
        let px = (player_minimap_x as f64 * (1.0 - t) + end_x * t) as isize;
        let py = (player_minimap_y as f64 * (1.0 - t) + end_y * t) as isize;

        if px >= 0 && px < WIDTH as isize && py >= 0 && py < HEIGTH as isize {
            buffer[py as usize * WIDTH + px as usize] = 0x00FF00;
        }
    }

    for i in 0..minimap_size {
        if offset_y < HEIGTH {
            if offset_x + i < WIDTH {
                buffer[offset_y * WIDTH + offset_x + i] = 0xFFFFFF;
                buffer[(offset_y + minimap_size - 1) * WIDTH + offset_x + i] = 0xFFFFFF;
            }
        }

        if offset_x < WIDTH {
            if offset_y + i < HEIGTH {
                buffer[(offset_y + i) * WIDTH + offset_x] = 0xFFFFFF;
                buffer[(offset_y + i) * WIDTH + offset_y + minimap_size - 1] = 0xFFFFFF;
            }
        }
    }
}

pub fn draw_hud(player: &Player, fps: u32, buffer: &mut [u32]) {
    let info_text = [
        format!("FPS: {}", fps),
        format!("Pos: ({:.2}, {:.2})", player.x, player.y),
        format!("Angle: {:.1}º", player.angle),
    ];

    for (i, text) in info_text.iter().enumerate() {
        draw_text(text, 10, 10 + i * 15, 0xFFFFFF, buffer);
    }
}

fn draw_text(text: &str, x: usize, y: usize, color: u32, buffer: &mut [u32]) {
    for (i, c) in text.chars().enumerate() {
        if x + i * 8 < WIDTH && y < HEIGTH {
            match c {
                '0'..='9' | 'A'..='Z' | 'a'..='z' | ':' | '(' | ')' | '.' | ',' | '-' => {
                    for dy in 0..8 {
                        for dx in 0..8 {
                            let px = x + i * 8 + dx;
                            let py = y + dy;
                            if px < WIDTH && py < HEIGTH {
                                if (dx > 0 && dx < 7) || (dy > 0 && dy < 7) {
                                    buffer[py * WIDTH + py] = color
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
