use bevy::prelude::*;
mod player;
use player::PlayerPlugin;

// RETICLE
const RETICLE_DIAMETER: f32 = 20.;
const RETICLE_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

// WALL
const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, move_reticle)
        .run();
}

#[derive(Component, Default)]
pub struct Collider;

#[derive(Component)]
struct Reticle;

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
struct Wall;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl Wall {
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: location.position().extend(0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: location.size().extend(1.0),
                ..default()
            },
        )
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>,
) {
    for mut window in windows.iter_mut() {
        window.cursor_options.visible = false;
    }
    // Camera
    commands.spawn(Camera2d);

    // Reticle
    let mut spawnpos = Vec3::new(0.0, 0.0, 101.0);

    if let Ok(windows) = windows.single() {
        if let Some(position) = windows.cursor_position() {
            spawnpos = Vec3::new(position.x, position.y, 0.0);
        }
    }

    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(
            RETICLE_DIAMETER / 2. - 1.0,
            RETICLE_DIAMETER / 2.,
        ))),
        MeshMaterial2d(materials.add(RETICLE_COLOR)),
        Transform::from_translation(spawnpos),
        Reticle,
    ));

    // Walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
}

fn move_reticle(
    windows: Query<&mut Window>,
    mut reticle_transform: Single<&mut Transform, With<Reticle>>,
) {
    if let Ok(window) = windows.single() {
        if let Some(position) = window.cursor_position() {
            let x = position.x - window.width() / 2.;
            let y = window.height() / 2. - position.y;
            reticle_transform.translation.x = x;
            reticle_transform.translation.y = y;
        }
    }
}
