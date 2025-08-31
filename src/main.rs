use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::{Duration, Instant};

struct GameMap {
    width: usize,
    height: usize,
    grid: Vec<Vec<u8>>,
}

impl GameMap {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
            grid: vec![
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1,
                    0, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
                    0, 0, 0, 1, 0, 1,
                ],
                vec![
                    1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1,
                    1, 1, 0, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1,
                    0, 1, 1, 1, 1, 1,
                ],
                vec![
                    1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                    0, 0, 0, 1, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1,
                    1, 1, 0, 1, 0, 1,
                ],
                vec![
                    1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                    0, 1, 0, 1, 0, 1,
                ],
                vec![
                    1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
                    0, 1, 0, 1, 0, 1,
                ],
                vec![
                    1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                    0, 1, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,
                    1, 1, 1, 1, 0, 1,
                ],
                vec![
                    1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                    1, 0, 0, 0, 0, 1,
                ],
                vec![
                    1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1,
                    1, 0, 1, 1, 1, 1,
                ],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1,
                ],
            ],
        }
    }

    fn is_wall(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return true;
        }
        self.grid[y][x] != 0
    }
}

struct Player {
    x: f64,
    y: f64,
    angle: f64,
    fov: f64,
    speed: f64,
    rot_speed: f64,
    health: i32,
}

impl Player {
    fn new(x: f64, y: f64, angle: f64) -> Self {
        Self {
            x,
            y,
            angle,
            fov: 60.0,
            speed: 0.05,
            rot_speed: 3.0,
            health: 100,
        }
    }

    fn update(&mut self, map: &GameMap, window: &Window) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if window.is_key_down(Key::W) {
            dx += self.angle.to_radians().cos() * self.speed;
            dy += self.angle.to_radians().sin() * self.speed;
        }
        if window.is_key_down(Key::S) {
            dx -= self.angle.to_radians().cos() * self.speed;
            dy -= self.angle.to_radians().sin() * self.speed;
        }
        if window.is_key_down(Key::A) {
            dx += (self.angle - 90.0).to_radians().cos() * self.speed;
            dy += (self.angle - 90.0).to_radians().sin() * self.speed;
        }
        if window.is_key_down(Key::D) {
            dx += (self.angle + 90.0).to_radians().cos() * self.speed;
            dy += (self.angle + 90.0).to_radians().sin() * self.speed;
        }

        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if !map.is_wall(new_x as usize, self.y as usize) {
            self.x = new_x;
        }
        if !map.is_wall(self.x as usize, new_y as usize) {
            self.y = new_y;
        }

        if window.is_key_down(Key::Left) {
            self.angle -= self.rot_speed;
        }
        if window.is_key_down(Key::Right) {
            self.angle += self.rot_speed;
        }

        self.angle = normalize_angle(self.angle);
    }

    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }
}

struct Enemy {
    x: f64,
    y: f64,
    health: i32,
    speed: f64,
    angle: f64,
    last_decision: usize,
    state: EnemyState,
    detection_range: f64,
}

#[derive(PartialEq)]
enum EnemyState {
    Patrolling,
    Chasing,
    Confused,
}

impl Enemy {
    fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x,
            y,
            health: 100,
            speed: 0.02,
            angle: rng.gen_range(0.0..360.0),
            last_decision: 0,
            state: EnemyState::Patrolling,
            detection_range: 5.0,
        }
    }

    fn update(&mut self, player_x: f64, player_y: f64, map: &GameMap, frame_count: usize) {
        let dx = player_x - self.x;
        let dy = player_y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < self.detection_range && self.has_line_of_sight(player_x, player_y, map) {
            self.state = EnemyState::Chasing;
        } else if self.state == EnemyState::Chasing {
            self.state = EnemyState::Confused;
        }

        if frame_count - self.last_decision > 30 {
            self.last_decision = frame_count;

            match self.state {
                EnemyState::Patrolling => self.patrol(map),
                EnemyState::Chasing => self.chase(player_x, player_y, map),
                EnemyState::Confused => {
                    self.wander(map);
                    if rand::random::<f32>() < 0.05 {
                        self.state = EnemyState::Patrolling;
                    }
                }
            }
        }

        self.move_enemy(map);
    }

    fn has_line_of_sight(&self, target_x: f64, target_y: f64, map: &GameMap) -> bool {
        let mut x = self.x;
        let mut y = self.y;
        let dx = target_x - x;
        let dy = target_y - y;
        let distance = (dx * dx + dy * dy).sqrt();
        let steps = (distance * 10.0) as usize;

        for i in 0..steps {
            let t = i as f64 / steps as f64;
            x = self.x + dx * t;
            y = self.y + dy * t;

            if map.is_wall(x as usize, y as usize) {
                return false;
            }
        }

        true
    }

    fn patrol(&mut self, map: &GameMap) {
        let mut rng = rand::thread_rng();

        if rng.r#gen::<f32>() < 0.2 {
            self.angle += rng.gen_range(-45.0..45.0);
            self.angle = normalize_angle(self.angle);
        }

        if rng.r#gen::<f32>() < 0.05 {
            self.speed = 0.0;
        } else {
            self.speed = 0.02;
        }
    }

    fn chase(&mut self, player_x: f64, player_y: f64, map: &GameMap) {
        let dx = player_x - self.x;
        let dy = player_y - self.y;
        self.angle = normalize_angle(dy.atan2(dx).to_degrees());
        self.speed = 0.03;
    }

    fn wander(&mut self, map: &GameMap) {
        let mut rng = rand::thread_rng();
        self.angle += rng.gen_range(-90.0..90.0);
        self.angle = normalize_angle(self.angle);
        self.speed = 0.015;
    }

    fn move_enemy(&mut self, map: &GameMap) {
        if self.speed > 0.0 {
            let new_x = self.x + self.angle.to_radians().cos() * self.speed;
            let new_y = self.y + self.angle.to_radians().sin() * self.speed;

            if !map.is_wall(new_x as usize, self.y as usize) {
                self.x = new_x;
            }

            if !map.is_wall(self.x as usize, new_y as usize) {
                self.y = new_y;
            }

            if map.is_wall(new_x as usize, new_y as usize) {
                self.angle = normalize_angle(self.angle + 180.0);
            }
        }
    }

    fn render(&self, buffer: &mut [u32], player_x: f64, player_y: f64, player_angle: f64) {
        let dx = self.x - player_x;
        let dy = self.y - player_y;
        let distance = (dx * dx + dy * dy).sqrt();

        let mut angle = dy.atan2(dx).to_degrees() - player_angle;
        angle = normalize_angle(angle);

        if angle < 60.0 || angle > 300.0 {
            let screen_x = ((angle / 60.0) * 400.0 + 200.0) as usize;
            let size = (30.0 / distance) as usize;

            for y in 0..size {
                for x in 0..size {
                    let px = screen_x + x - size / 2;
                    let py = 300 + y - size / 2;

                    if px < 800 && py < 600 {
                        buffer[py * 800 + px] = 0xFF0000;
                    }
                }
            }
        }
    }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn normalize_angle(angle: f64) -> f64 {
    let mut a = angle;
    while a < 0.0 {
        a += 360.0;
    }
    while a >= 360.0 {
        a -= 360.0;
    }
    a
}

fn render_scene(player: &Player, map: &GameMap, buffer: &mut [u32]) {
    let fov = player.fov;
    let num_rays = 800;
    let angle_step = fov / num_rays as f64;

    for x in 0..num_rays {
        let ray_angle = player.angle - (fov / 2.0) + (x as f64 * angle_step);
        let ray_angle_rad = ray_angle.to_radians();

        let ray_dir_x = ray_angle_rad.cos();
        let ray_dir_y = ray_angle_rad.sin();

        let mut ray_x = player.x;
        let mut ray_y = player.y;

        let mut distance = 0.0;
        let mut hit_wall = false;

        while !hit_wall && distance < 20.0 {
            ray_x += ray_dir_x * 0.01;
            ray_y += ray_dir_y * 0.01;
            distance += 0.01;

            if map.is_wall(ray_x as usize, ray_y as usize) {
                hit_wall = true;
            }
        }

        let wall_height = (600.0 / distance) as usize;
        let start = if 300 >= wall_height / 2 {
            300 - wall_height / 2
        } else {
            0
        };
        let end = if 300 + wall_height / 2 < 600 {
            300 + wall_height / 2
        } else {
            600 - 1
        };

        for y in start..end {
            let shade = if distance < 2.0 {
                0xFFFFFF
            } else if distance < 4.0 {
                0xAAAAAA
            } else if distance < 7.0 {
                0x777777
            } else if distance < 13.5 {
                0x333333
            } else {
                0x000000
            };

            buffer[y * 800 + x] = shade;
        }

        for y in 0..start {
            buffer[y * 800 + x] = 0x2222AA;
        }

        for y in end..600 {
            buffer[y * 800 + x] = 0x22AA22;
        }
    }

    if player.health < 30 {
        let intensity = (30 - player.health) as f32 / 30.0;
        for i in 0..buffer.len() {
            let r = ((buffer[i] >> 16) & 0xFF) as f32;
            let g = ((buffer[i] >> 8) & 0xFF) as f32;
            let b = (buffer[i] & 0xFF) as f32;

            let new_r = (r * (1.0 - intensity) + 255.0 * intensity) as u32;
            let new_g = (g * (1.0 - intensity)) as u32;
            let new_b = (b * (1.0 - intensity)) as u32;

            buffer[i] = (new_r << 16) | (new_g << 8) | new_b;
        }
    }
}

fn draw_minimap(player: &Player, map: &GameMap, buffer: &mut [u32]) {
    let scale = 6;
    let minimap_size = 150;
    let offset_x = 800 - minimap_size - 10;
    let offset_y = 10;

    for y in 0..minimap_size {
        for x in 0..minimap_size {
            let px = offset_x + x;
            let py = offset_y + y;
            if px < 800 && py < 600 {
                let current_color = buffer[py * 800 + px];
                let r = ((current_color >> 16) & 0xFF) / 2;
                let g = ((current_color >> 8) & 0xFF) / 2;
                let b = (current_color & 0xFF) / 2;
                buffer[py * 800 + px] = (r << 16) | (g << 8) | b;
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

                    if px < 800 && py < 600 {
                        buffer[py * 800 + px] = 0x8888FF;
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
            if dx * dx + dy * dy <= player_radius * player_radius {
                let px = player_minimap_x as isize + dx;
                let py = player_minimap_y as isize + dy;

                if px >= 0 && px < 800 as isize && py >= 0 && py < 600 as isize {
                    buffer[py as usize * 800 + px as usize] = 0xFF0000;
                }
            }
        }
    }

    let direction_length = 10;
    let end_x = player_minimap_x as f64 + player.angle.to_radians().cos() * direction_length as f64;
    let end_y = player_minimap_y as f64 + player.angle.to_radians().sin() * direction_length as f64;

    let steps = direction_length * 2;
    for i in 0..steps {
        let t = i as f64 / steps as f64;
        let px = (player_minimap_x as f64 * (1.0 - t) + end_x * t) as isize;
        let py = (player_minimap_y as f64 * (1.0 - t) + end_y * t) as isize;

        if px >= 0 && px < 800 as isize && py >= 0 && py < 600 as isize {
            buffer[py as usize * 800 + px as usize] = 0x00FF00;
        }
    }

    for i in 0..minimap_size {
        if offset_y < 600 {
            if offset_x + i < 800 {
                buffer[offset_y * 800 + offset_x + i] = 0xFFFFFF;
                buffer[(offset_y + minimap_size - 1) * 800 + offset_x + i] = 0xFFFFFF;
            }
        }

        if offset_x < 800 {
            if offset_y + i < 600 {
                buffer[(offset_y + i) * 800 + offset_x] = 0xFFFFFF;
                buffer[(offset_y + i) * 800 + offset_x + minimap_size - 1] = 0xFFFFFF;
            }
        }
    }
}

// ============ HUD ============
fn draw_hud(player: &Player, fps: u32, enemies: &[Enemy], buffer: &mut [u32]) {
    let info_text = [
        format!("FPS: {}", fps),
        format!("Pos: ({:.2}, {:.2})", player.x, player.y),
        format!("Angle: {:.1}°", player.angle),
        format!("Inimigos: {}", enemies.len()),
    ];

    for (i, text) in info_text.iter().enumerate() {
        draw_text(text, 10, 10 + i * 15, 0xFFFFFF, buffer);
    }

    let health_bar_width = 100;
    for x in 0..health_bar_width {
        for y in 0..5 {
            let px = 10 + x;
            let py = 70 + y;
            if px < 800 && py < 600 {
                let color = if x < health_bar_width * player.health as usize / 100 {
                    0x00FF00
                } else {
                    0x333333
                };
                buffer[py * 800 + px] = color;
            }
        }
    }
}

fn draw_text(text: &str, x: usize, y: usize, color: u32, buffer: &mut [u32]) {
    for (i, c) in text.chars().enumerate() {
        let char_x = x + i * 8;
        match c {
            '0'..='9' | 'A'..='Z' | 'a'..='z' | ':' | '(' | ')' | '.' | ',' | '-' | ' ' => {
                for dy in 0..8 {
                    for dx in 0..8 {
                        let px = char_x + dx;
                        let py = y + dy;
                        if px < 800 && py < 600 && (dx > 0 && dx < 7) && (dy > 0 && dy < 7) {
                            buffer[py * 800 + px] = color;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut window =
        Window::new("Labirinto Assombrado", 800, 600, WindowOptions::default()).unwrap();
    let mut buffer: Vec<u32> = vec![0; 800 * 600];

    let mut player = Player::new(1.5, 1.5, 0.0);
    let map = GameMap::default();

    let mut enemies = vec![
        Enemy::new(5.5, 5.5),
        Enemy::new(10.5, 10.5),
        Enemy::new(15.5, 15.5),
        Enemy::new(20.5, 20.5),
    ];

    let mut frame_count = 0;
    let mut fps_counter = 0;
    let mut fps = 0;
    let mut last_fps_update = Instant::now();
    let fps_update_interval = Duration::from_secs(1);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0;
        }

        player.update(&map, &window);

        for enemy in &mut enemies {
            enemy.update(player.x, player.y, &map, frame_count);
        }

        render_scene(&player, &map, &mut buffer);

        for enemy in &enemies {
            enemy.render(&mut buffer, player.x, player.y, player.angle);
        }

        draw_minimap(&player, &map, &mut buffer);

        fps_counter += 1;
        if last_fps_update.elapsed() >= fps_update_interval {
            fps = fps_counter;
            fps_counter = 0;
            last_fps_update = Instant::now();
        }

        draw_hud(&player, fps, &enemies, &mut buffer);

        window.update_with_buffer(&buffer, 800, 600).unwrap();
        frame_count += 1;
    }
}
