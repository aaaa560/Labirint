use crate::map::GameMap;
use crate::player::Player;

const WIDTH: usize = 800;
const HEIGTH: usize = 600;

pub fn draw_minimap(player: &Player, map: &GameMap, buffer: &mut [u32]) {
    let scale = 8;
    for y in 0..map.height {
        for x in 0..map.width {
            let color = if map.grid[y][x] == 1 {
                0xFFFFFF
            } else {
                0x000000
            };
            for sy in 0..scale {
                for sx in 0..scale {
                    let px = x * scale + sx;
                    let py = y * scale + sy;
                    if px < WIDTH && py < HEIGTH {
                        buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }
    }

    let px = (player.x as usize) * scale;
    let py = (player.y as usize) * scale;

    for sy in 0..scale {
        for sx in 0..scale {
            let nx = px + sx;
            let ny = py + sy;
            if nx < WIDTH && ny < HEIGTH {
                buffer[ny * WIDTH + nx] = 0xFF0000;
            }
        }
    }
}
