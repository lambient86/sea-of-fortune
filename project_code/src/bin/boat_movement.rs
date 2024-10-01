use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "Boat Test";
const BOUNDS: Vec2 = Vec2::new(1920.0, 1080.0);
const TILE_SIZE: u32 = 100;
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const LEVEL_W: f32 = 1920.;
const LEVEL_H: f32 = 1080.;

enum PlayerType {
    Boat,
}

use bevy::color::palettes::css::{BLUE, LIGHT_BLUE};

// const ACCEL_RATE: f32 = 3600.;

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Component)]
struct Background;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(LIGHT_BLUE)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                boat_movement,
            ),
        )
        .add_systems(Update, move_camera.after(boat_movement))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    let bg_texture_handle = asset_server.load("ocean_demo.png");

    commands
        .spawn(SpriteBundle {
            texture: bg_texture_handle.clone(),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(Background);

    let boat_sheet_handle = asset_server.load("basic_ship.png");
    let boat_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let boat_layout_handle = texture_atlases.add(boat_layout);
    commands.spawn((
        SpriteBundle {
            texture: boat_sheet_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: boat_layout_handle.clone(),
            index: PlayerType::Boat as usize,
        },
        Player{
            movement_speed: 250.0,
            rotation_speed: f32::to_radians(180.0),
        },
    ));
}

fn boat_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement_factor += 1.0;
    }

    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    let movement_dir = transform.rotation * Vec3::Y;
    let movement_dis = movement_factor * ship.movement_speed * time.delta_seconds();
    let translation_delta = movement_dir * movement_dis;

    transform.translation += translation_delta;

    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;
    ct.translation.x = pt.translation.x.clamp(-x_bound, x_bound);
    ct.translation.y = pt.translation.y.clamp(-y_bound, y_bound);
}