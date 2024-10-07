use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

const TITLE: &str = "Bat Test";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SIZE: f32 = 32.;
const TILE_SIZE: u32 = 32;
const PLAYER_SPEED: f32 = 300.;
const ACCEL_RATE: f32 = 3600.;
const ANIM_TIME: f32 = 0.2;
const ATTACK_DIST: f32 = 200.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bat {
    rotation_speed: f32,
}

#[derive(Component)]
struct RotateToPlayer {
    rotation_speed: f32,
}

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

use bevy::color::palettes::css::SEA_GREEN;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, animate_bat)
        .add_systems(Update, rotate_bat)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::Srgba(SEA_GREEN),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(Velocity::new())
        .insert(Player);

    let bat_sheet_handle = asset_server.load("s_bat.png");
    let bat_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 3, 1, None, None);
    let bat_layout_len = 3;
    let bat_layout_handle = texture_atlases.add(bat_layout.clone());

    commands.spawn((
        SpriteBundle {
            texture: bat_sheet_handle,
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        Bat {
            rotation_speed: f32::to_radians(90.0),
        },
        RotateToPlayer {
            rotation_speed: f32::to_radians(90.0),
        },
        TextureAtlas {
            layout: bat_layout_handle,
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(bat_layout_len),
        Velocity::new(),
    ));

}   

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut pt, mut pv) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    pt.translation.x = (pt.translation.x + change.x).clamp(
        -(WIN_W / 2.) + PLAYER_SIZE / 2.,
        WIN_W / 2. - PLAYER_SIZE / 2.,
    );
    pt.translation.y = (pt.translation.y + change.y).clamp(
        -(WIN_H / 2.) + PLAYER_SIZE / 2.,
        WIN_H / 2. - PLAYER_SIZE / 2.,
    );
}

fn animate_bat(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &AnimationFrameCount)>,
) {
    for (mut timer, mut texture_atlas, frame_count) in &mut query {
        timer.tick(time.delta());

        if timer.finished() {
            texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
        }
    }
}

fn rotate_bat(
    time: Res<Time>,
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for (config, mut enemy_transform) in &mut query {
        let bat_position = enemy_transform.translation.xy();
        let distance_to_player = bat_position.distance(player_translation);

        if distance_to_player > ATTACK_DIST {
            continue;
        }


        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();

        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        let forward_dot_player = enemy_forward.dot(to_player);

        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        let right_dot_player = enemy_right.dot(to_player);

        let rotation_sign = -f32::copysign(1.0, right_dot_player);
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); 

        let rotation_angle =
            rotation_sign * (config.rotation_speed * time.delta_seconds()).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);
    }



}