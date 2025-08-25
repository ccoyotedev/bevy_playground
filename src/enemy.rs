use bevy::prelude::*;
use rand::prelude::*;

use crate::movable::Movable;

#[derive(Component)]
#[require(Movable)]
pub struct Enemy;

const ENEMY_DIAMETER: f32 = 40.0;
// const ENEMY_MAX_SPEED: f32 = 400.0;
// const ENEMY_ACCELERATION: f32 = 600.;
// Applied when there is no input; larger values stop faster
// const ENEMY_DAMPING: f32 = 2.0;
const ENEMY_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const ENEMIES_TO_SPAWN: i32 = 5;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let arena_width = 900.;
    let arena_height = 600.;
    let radius = ENEMY_DIAMETER / 2.0;

    let min_x = -arena_width / 2.0 + radius;
    let max_x = arena_width / 2.0 - radius;
    let min_y = -arena_height / 2.0 + radius;
    let max_y = arena_height / 2.0 - radius;

    for _ in 0..ENEMIES_TO_SPAWN {
        let mut rng = rand::rng();
        let random_x: f32 = min_x + rng.random::<f32>() * (max_x - min_x);
        let random_y: f32 = min_y + rng.random::<f32>() * (max_y - min_y);

        let starting_position: Vec3 = Vec3::new(random_x, random_y, 1.0);

        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(ENEMY_COLOR)),
            Transform::from_translation(starting_position)
                .with_scale(Vec2::splat(ENEMY_DIAMETER).extend(1.0)),
            Enemy,
            Movable::new(),
            crate::Collider,
        ));
    }
}
