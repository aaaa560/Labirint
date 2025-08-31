// raycast.rs
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
        let ray_angle_rad = ray_angle.to_radians();

        let ray_dir_x = ray_angle_rad.cos();
        let ray_dir_y = ray_angle_rad.sin();

        let mut ray_x = player.x;
        let mut ray_y = player.y;

        let mut distance = 0.0;

        let mut wall_direction = 0;

        let delta_dist_x = if ray_dir_x == 0.0 {
            1e30
        } else {
            (1.0 / ray_dir_x).abs()
        };
        let delta_dist_y = if ray_dir_y == 0.0 {
            1e30
        } else {
            (1.0 / ray_dir_y).abs()
        };

        let mut map_x = ray_x as i32;
        let mut map_y = ray_y as i32;

        let mut side_dist_x: f64;
        let mut side_dist_y: f64;

        let step_x: i32;
        let step_y: i32;

        if ray_dir_x < 0.0 {
            step_x = -1;
            side_dist_x = (ray_x - map_x as f64) * delta_dist_x;
        } else {
            step_x = 1;
            side_dist_x = (map_x as f64 + 1.0 - ray_x) * delta_dist_x;
        }

        if ray_dir_y < 0.0 {
            step_y = -1;
            side_dist_y = (ray_y - map_y as f64) * delta_dist_y;
        } else {
            step_y = 1;
            side_dist_y = (map_y as f64 + 1.0 - ray_y) * delta_dist_y;
        }

        let mut hit = false;
        while !hit && distance < 20.0 {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                wall_direction = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                wall_direction = 1;
            }
            if map_x >= 0 && map_x < map.width as i32 && map_y >= 0 && map_y < map.height as i32 {
                if map.grid[map_y as usize][map_x as usize] > 0 {
                    hit = true;
                }
            } else {
                break;
            }

            distance += 0.01;
        }

        let perp_wall_dist = if wall_direction == 0 {
            (map_x as f64 - ray_x + (1 - step_x) as f64 / 2.0) / ray_dir_x
        } else {
            (map_y as f64 - ray_y + (1 - step_y) as f64 / 2.0) / ray_dir_y
        }
        .abs();

        let line_height = (HEIGHT as f64 / perp_wall_dist) as usize;
        let draw_start = if line_height > HEIGHT {
            0
        } else {
            (HEIGHT - line_height) / 2
        };

        let draw_end = if line_height > HEIGHT {
            HEIGHT
        } else {
            draw_start + line_height
        };

        let base_color = match map.grid[map_y as usize][map_x as usize] {
            1 => 0xFF6B6B,
            _ => 0xFF6B6B,
        };

        // Corrigindo o erro de tipo - convertendo shade para f32
        let shade = (1.0 - (perp_wall_dist / 20.0).min(1.0)) * 0.8 + 0.2;
        let shade_f32 = shade as f32; // Convertendo para f32

        let (r, g, b) = (
            (((base_color >> 16) & 0xFF) as f32 * shade_f32) as u32,
            (((base_color >> 8) & 0xFF) as f32 * shade_f32) as u32,
            ((base_color & 0xFF) as f32 * shade_f32) as u32,
        );

        let color = (r << 16) | (g << 8) | b;

        for y in draw_start..draw_end {
            buffer[y * WIDTH + x] = color;
        }

        for y in 0..draw_start {
            let ceiling_shade = 1.0 - (y as f32 / HEIGHT as f32) * 0.5;
            // Convertendo para f32 explicitamente
            let r_ceil = (100.0f32 * ceiling_shade) as u32;
            let g_ceil = (120.0f32 * ceiling_shade) as u32;
            let b_ceil = (200.0f32 * ceiling_shade) as u32;
            buffer[y * WIDTH + x] = (r_ceil << 16) | (g_ceil << 8) | b_ceil;
        }

        for y in draw_end..HEIGHT {
            let floor_shade = 0.5 + (y as f32 / HEIGHT as f32) * 0.5;
            // Convertendo para f32 explicitamente
            let r_floor = (139.0f32 * floor_shade) as u32;
            let g_floor = (69.0f32 * floor_shade) as u32;
            let b_floor = (19.0f32 * floor_shade) as u32;
            buffer[y * WIDTH + x] = (r_floor << 16) | (g_floor << 8) | b_floor;
        }
    }
}
