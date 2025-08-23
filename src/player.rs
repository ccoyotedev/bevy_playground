use bevy::{
    math::ops::{atan2, sqrt},
    prelude::*,
};

use crate::movable::Movable;

// Player configuration
const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const PLAYER_DIAMETER: f32 = 50.0;
const PLAYER_MAX_SPEED: f32 = 400.0;
const PLAYER_ACCELERATION: f32 = 600.;
// Applied when there is no input; larger values stop faster
const PLAYER_DAMPING: f32 = 2.0;
const PLAYER_COLOR: Color = Color::srgb(0.5, 0.5, 1.);

// Sightline configuration
const SIGHTLINE_LENGTH: f32 = 200.0;
const SIGHTLINE_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

#[derive(Component)]
#[require(Movable)]
pub struct Player;

#[derive(Component)]
struct SightLine;

#[derive(Component)]
struct SightLineMesh;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, (move_player, move_sightline).chain());
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Player root
    let player_entity = commands
        .spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(PLAYER_COLOR)),
            Transform::from_translation(PLAYER_STARTING_POSITION)
                .with_scale(Vec2::splat(PLAYER_DIAMETER).extend(1.0)),
            Player,
            Movable::new(),
            crate::Collider,
        ))
        .id();

    // Sight Line (pivot child of player; actual line is child of pivot)
    commands.entity(player_entity).with_children(|parent| {
        parent
            .spawn((
                // Pivot sits at player's center; rotate this. Apply inverse of player scale
                // so child geometry uses world units.
                Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)).with_scale(Vec3::new(
                    1.0 / PLAYER_DIAMETER,
                    1.0 / PLAYER_DIAMETER,
                    1.0,
                )),
                SightLine,
            ))
            .with_children(|pivot_parent| {
                pivot_parent.spawn((
                    // 1px tall, 1px wide in pivot-local (world) units
                    Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                    MeshMaterial2d(materials.add(SIGHTLINE_COLOR)),
                    // Scale X to length each frame; translate to half-length
                    Transform::from_translation(Vec3::new(0.5, 0.0, 0.0)),
                    SightLineMesh,
                ));
            });
    });
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &mut Movable), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut movable)) = player_q.single_mut() else {
        return;
    };

    // Input acceleration from WASD
    let mut accel_input = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        accel_input.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        accel_input.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        accel_input.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        accel_input.x += 1.0;
    }

    // Update acceleration and velocity
    let accel_world = accel_input.normalize_or_zero() * PLAYER_ACCELERATION;
    movable.velocity += accel_world * time.delta_secs();

    // Per-axis damping: if there's no input on an axis, damp that axis
    let factor = (1.0 - PLAYER_DAMPING * time.delta_secs()).max(0.0);
    if accel_input.x == 0.0 {
        movable.velocity.x *= factor;
        if movable.velocity.x.abs() < 1.0 {
            movable.velocity.x = 0.0;
        }
    }
    if accel_input.y == 0.0 {
        movable.velocity.y *= factor;
        if movable.velocity.y.abs() < 1.0 {
            movable.velocity.y = 0.0;
        }
    }

    // Clamp max speed
    let speed = movable.velocity.length();
    if speed > PLAYER_MAX_SPEED {
        movable.velocity = movable.velocity / speed * PLAYER_MAX_SPEED;
    }

    // Integrate position
    transform.translation.x += movable.velocity.x * time.delta_secs();
    transform.translation.y += movable.velocity.y * time.delta_secs();
}

fn move_sightline(
    windows: Query<&Window>,
    player_transform: Single<&GlobalTransform, With<Player>>,
    mut pivot_q: Query<&mut Transform, (With<SightLine>, Without<SightLineMesh>)>,
    mut mesh_q: Query<&mut Transform, (With<SightLineMesh>, Without<SightLine>)>,
) {
    if let Ok(window) = windows.single() {
        if let Some(position) = window.cursor_position() {
            let mouse_x = position.x - window.width() / 2.0;
            let mouse_y = window.height() / 2.0 - position.y;

            let player_pos = player_transform.translation();
            let dy = mouse_y - player_pos.y;
            let dx = mouse_x - player_pos.x;
            let hyp = sqrt(dy * dy + dx * dx);
            let clamped = hyp.min(SIGHTLINE_LENGTH);

            let angle = atan2(dy, dx);

            if let (Ok(mut pivot_transform), Ok(mut mesh_transform)) =
                (pivot_q.single_mut(), mesh_q.single_mut())
            {
                // Rotate pivot to face the mouse
                pivot_transform.rotation = Quat::from_rotation_z(angle);
                // Scale child geometry length to the clamped distance and place it half-way along X
                mesh_transform.scale.x = clamped;
                mesh_transform.translation.x = clamped * 0.5;
            }
        }
    }
}
