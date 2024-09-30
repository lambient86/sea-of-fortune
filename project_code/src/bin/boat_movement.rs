use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "Boat Test";
const BOUNDS: Vec2 = Vec2::new(1280.0, 720.0);

use bevy::color::palettes::css::{BLUE, LIGHT_BLUE};

// const ACCEL_RATE: f32 = 3600.;

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}


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
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let ship_handle = asset_server.load("basic_ship.png");

    commands.spawn((
        SpriteBundle {
            texture: ship_handle,
            ..default()
        },
        Player {
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