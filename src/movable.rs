use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Movable {
    pub velocity: Vec2,
}

impl Movable {
    pub fn new() -> Movable {
        Movable {
            velocity: Vec2::ZERO,
        }
    }

    pub fn apply_acceleration(&mut self, direction: Vec2, acceleration: f32, delta_seconds: f32) {
        let accel_world = direction.normalize_or_zero() * acceleration;
        self.velocity += accel_world * delta_seconds;
    }

    pub fn apply_axis_damping(&mut self, input_direction: Vec2, damping: f32, delta_seconds: f32) {
        let factor = (1.0 - damping * delta_seconds).max(0.0);
        if input_direction.x == 0.0 {
            self.velocity.x *= factor;
            if self.velocity.x.abs() < 1.0 {
                self.velocity.x = 0.0;
            }
        }
        if input_direction.y == 0.0 {
            self.velocity.y *= factor;
            if self.velocity.y.abs() < 1.0 {
                self.velocity.y = 0.0;
            }
        }
    }

    pub fn clamp_max_speed(&mut self, max_speed: f32) {
        let speed = self.velocity.length();
        if speed > max_speed {
            self.velocity = self.velocity / speed * max_speed;
        }
    }

    pub fn integrate_position(&self, transform: &mut Transform, delta_seconds: f32) {
        transform.translation.x += self.velocity.x * delta_seconds;
        transform.translation.y += self.velocity.y * delta_seconds;
    }
}
