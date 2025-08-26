use bevy::prelude::*;
use rand::prelude::*;

use crate::movable::Movable;
use crate::player::Player;

const ENEMY_DIAMETER: f32 = 40.0;
// const ENEMY_MAX_SPEED: f32 = 400.0;
// const ENEMY_ACCELERATION: f32 = 600.;
// Applied when there is no input; larger values stop faster
// const ENEMY_DAMPING: f32 = 2.0;
const ENEMY_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const ENEMIES_TO_SPAWN: i32 = 5;
const ENEMY_ACCELERATION: f32 = 400.;
const ENEMY_MAX_SPEED: f32 = 300.0;
const ENEMY_DAMPING: f32 = 2.0;

#[derive(Component)]
#[require(Movable)]
pub struct Enemy {}

impl Enemy {
    fn new() -> Enemy {
        Enemy {}
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
        app.add_systems(FixedUpdate, enemy_movement);
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
            Enemy::new(),
            Movable::new(),
            crate::Collider,
        ));
    }
}

fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Movable), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single_mut() else {
        return;
    };

    for (mut transform, mut movable) in enemy_query.iter_mut() {
        let player_pos = player_transform.translation;
        let enemy_pos = transform.translation;
        let dy = player_pos.y - enemy_pos.y;
        let dx = player_pos.x - enemy_pos.x;
        let accel_dir = Vec2::new(dx, dy);

        // Update acceleration and velocity
        movable.apply_acceleration(accel_dir, ENEMY_ACCELERATION, time.delta_secs());

        // Per-axis damping: if there's no input on an axis, damp that axis
        movable.apply_axis_damping(accel_dir, ENEMY_DAMPING, time.delta_secs());

        // Clamp max speed
        movable.clamp_max_speed(ENEMY_MAX_SPEED);

        // Integrate position
        movable.integrate_position(&mut transform, time.delta_secs());
    }
}
