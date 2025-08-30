use crate::map::GameMap;
use crate::utils::normalize_angle;
use minifb::{Key, Window};

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub fov: f64,
    pub speed: f64,
    pub rot_speed: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Self {
            x,
            y,
            angle,
            fov: 60.0,
            speed: 0.05,
            rot_speed: 3.0,
        }
    }

    pub fn update(&mut self, map: &GameMap, window: &Window) {
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
}
