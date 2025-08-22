use bevy::prelude::*;

// PLAYER
const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const PLAYER_DIAMETER: f32 = 50.;
const PLAYER_SPEED: f32 = 400.0;
const PLAYER_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_player)
        .add_systems(Update, move_reticle)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Collider;

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

    // Player
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(PLAYER_COLOR)),
        Transform::from_translation(PLAYER_STARTING_POSITION)
            .with_scale(Vec2::splat(PLAYER_DIAMETER).extend(1.)),
        Player,
        Collider,
    ));

    // Reticle
    let mut spawnpos = Vec3::new(0.0, 0.0, 101.0);

    if let Ok(windows) = windows.single() {
        if let Some(position) = windows.cursor_position() {
            spawnpos = Vec3::new(position.x, position.y, 0.0);
        }
    }

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(RETICLE_COLOR)),
        Transform::from_translation(spawnpos).with_scale(Vec2::splat(RETICLE_DIAMETER).extend(1.)),
        Reticle,
    ));

    // Walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    let is_up = keyboard_input.pressed(KeyCode::KeyW);
    let is_down = keyboard_input.pressed(KeyCode::KeyS);
    let is_left = keyboard_input.pressed(KeyCode::KeyA);
    let is_right = keyboard_input.pressed(KeyCode::KeyD);

    if is_up {
        direction.y += 1.;
    }
    if is_down {
        direction.y -= 1.;
    }
    if is_left {
        direction.x -= 1.;
    }
    if is_right {
        direction.x += 1.;
    }

    let direction = direction.normalize_or_zero();

    let new_position = Vec2::new(
        player_transform.translation.x + direction.x * PLAYER_SPEED * time.delta_secs(),
        player_transform.translation.y + direction.y * PLAYER_SPEED * time.delta_secs(),
    );

    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_DIAMETER / 2.0;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_DIAMETER / 2.0;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLAYER_DIAMETER / 2.0;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLAYER_DIAMETER / 2.0;

    player_transform.translation.x = new_position.x.clamp(left_bound, right_bound);
    player_transform.translation.y = new_position.y.clamp(bottom_bound, top_bound);
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
