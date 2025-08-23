use bevy::prelude::*;

use crate::movable::Movable;

#[derive(Component)]
#[require(Movable)]
pub struct Enemy;

const ENEMY_STARTING_POSITION: Vec3 = Vec3::new(200., 200., 1.0);
const ENEMY_DIAMETER: f32 = 40.0;
// const ENEMY_MAX_SPEED: f32 = 400.0;
// const ENEMY_ACCELERATION: f32 = 600.;
// Applied when there is no input; larger values stop faster
// const ENEMY_DAMPING: f32 = 2.0;
const ENEMY_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ENEMY_COLOR)),
        Transform::from_translation(ENEMY_STARTING_POSITION)
            .with_scale(Vec2::splat(ENEMY_DIAMETER).extend(1.0)),
        Enemy,
        Movable::new(),
        crate::Collider,
    ));
}
