use bevy::{prelude::*, window::PresentMode};

// use crate::TILE_SIZE;

// //setting window constants
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const ACCELERATION: f32 = 3600.;
const PLAYER_SPEED: f32 = 300.;
const PLAYER_SIZE: f32 = 64.;
const TITLE: &str = "Player Movement Test";
const TILE_SIZE: u32 = 32;

const ANIMATION_TIME: f32 = 0.2;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    v: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            v: Vec2::splat(0.),     // set x and y values to o
        }
    }
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: TITLE.into(),
            resolution: (1280., 720.).into(),
            present_mode: PresentMode::Fifo,
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup)
    .add_systems(Update, player_movement)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pirateStatic-export.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        ..default()

    })
    .insert(Velocity::new())
    .insert(Player);

    let idle_handle: Handle<Image> = asset_server.load("pirateIdle-Sheet.png");
    let idle_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 15, 1, None, None);
    let idle_layout_length = idle_layout.textures.len();
    let idle_layout_handle = texture_atlases.add(idle_layout);

    commands.spawn((
      SpriteBundle {
        texture: idle_handle,
        ..default()
      },
      TextureAtlas {
        layout: idle_layout_handle,
        index: 0,
      },
      AnimationTimer(Timer::from_seconds((ANIMATION_TIME), TimerMode::Repeating)),
      AnimationFrameCount(idle_layout_length),
      Velocity::new(),
      Player,
    ));
}

pub fn player_movement(
    /*
    - query for player, time, keyboard input
    - also need to get mutable reference to player transformation and velocity
    - use velocity equation => v_final = v_initial + change_in_velocity
        - where change_in_velocity is acceleration * change_in_time
    */

    time: Res<Time>, 
    key_pressed: Res<ButtonInput<KeyCode>>, 
    mut player: Query<(&mut Transform, &mut Velocity),With<Player>>,) {

    let (mut player_transform, mut player_velocity) = player.single_mut();

    let mut delta_v = Vec2::splat(0.);

    // check which key was pressed and increase change in velocity in that direction

    if key_pressed.pressed(KeyCode::KeyW) {delta_v.y+=1.0;}      // increase velocity in +y

    if key_pressed.pressed(KeyCode::KeyA) {delta_v.x -=1.0;}     // increase velocity in -x

    if key_pressed.pressed(KeyCode::KeyS) {delta_v.y-=1.0;}      // increase velocity in -y

    if key_pressed.pressed(KeyCode::KeyD) {delta_v.x+=1.0;}      // increase velocity in +x

    let delta_t = time.delta_seconds();
    let acc = ACCELERATION * delta_t;

    player_velocity.v = if delta_v.length() > 0. {
     (player_velocity.v + (delta_v.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if player_velocity.v.length() > acc {
        (player_velocity.v + (player_velocity.v.normalize_or_zero() * -acc)).clamp_length_max(PLAYER_SPEED)
    } else {
        Vec2::splat(0.)
    };

    player_velocity.v = player_velocity.v + (acc * player_velocity.v);     // new velocity 

    let change_in_distance = player_velocity.v * delta_t;

    player_transform.translation.x = (player_transform.translation.x + change_in_distance.x).clamp(
        -(WIN_W / 2.) + PLAYER_SIZE / 2.,
        WIN_W / 2. - PLAYER_SIZE / 2.);
    player_transform.translation.y = (player_transform.translation.y + change_in_distance.y).clamp(
        -(WIN_H / 2.) + PLAYER_SIZE / 2.,
        WIN_H / 2. - PLAYER_SIZE / 2.);

}

fn player_animation() {

}