use crate::map::GameMap;
use crate::player::Player;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn render_scene(player: &Player, map: &GameMap, buffer: &mut [u32]) {
    let fov = player.fov;
    let num_rays = WIDTH;
    let angle_step = fov / num_rays as f64;

    for x in 0..num_rays {
        let ray_angle = player.angle - (fov / 2.0) + (x as f64 * angle_step);

        let mut ray_x = player.x;
        let mut ray_y = player.y;

        let step_size = 0.01;
        let mut distance = 0.0;
        let mut hit_wall = false;

        while !hit_wall && distance < 20.0 {
            ray_x += ray_angle.to_radians().cos() * step_size;
            ray_y += ray_angle.to_radians().sin() * step_size;
            distance += step_size;

            if map.is_wall(ray_x as usize, ray_y as usize) {
                hit_wall = true;
            }
        }

        let wall_height = (HEIGHT as f64 / distance) as usize;
        let start = if HEIGHT / 2 >= wall_height / 2 {
            HEIGHT / 2 - wall_height / 2
        } else {
            0
        };
        let end = if HEIGHT / 2 + wall_height / 2 < HEIGHT {
            HEIGHT / 2 + wall_height / 2
        } else {
            HEIGHT - 1
        };

        for y in start..end {
            let shade = if distance < 2.0 {
                0xFFFFFF
            } else if distance < 5.0 {
                0xAAAAAA
            } else if distance < 10.0 {
                0x777777
            } else {
                0x333333
            };

            buffer[y * WIDTH + x] = shade;
        }
    }
}
