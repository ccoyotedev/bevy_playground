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
}
