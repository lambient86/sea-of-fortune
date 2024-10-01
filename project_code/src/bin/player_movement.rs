use bevy::{math::VectorSpace, prelude::*, render::texture, window::PresentMode};

// use crate::TILE_SIZE;

// //setting window constants
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const ACCELERATION: f32 = 5000.;
const PLAYER_SPEED: f32 = 500.;
const PLAYER_SIZE: f32 = 32.;
const TITLE: &str = "Player Movement Test";
const TILE_SIZE: u32 = 32;
const LEVEL_W: f32 = 1920.;
const LEVEL_H: f32 = 1080.;

const ANIMATION_TIME: f32 = 0.1;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

#[derive(Component)]
pub struct Player {
    animation_state: SpriteState,
    timer: Timer,
}

impl SpriteState {
    fn animation_indices(&self) -> std::ops::Range<usize> {
        match self {
            SpriteState::Idle => 0..8,
            SpriteState::LeftRun => 8..16,
            SpriteState::RightRun => 16..24,
            SpriteState::ForwardRun => 24..32,
            SpriteState::BackwardRun => 32..40
        }
    }

    fn animation_speed(&self) -> f32 {
        match self {
            SpriteState::Idle => 0.1,
            SpriteState::LeftRun => 0.1,
            SpriteState::RightRun => 0.1,
            SpriteState::ForwardRun => 0.1,
            SpriteState::BackwardRun => 0.1,
        }
    }
} 
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum SpriteState {
        Idle,
        LeftRun,
        RightRun,
        ForwardRun,
        BackwardRun
}

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
    .add_systems(Update, move_player)
    .add_systems(Update, player_animation.after(move_player))
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>) {
    commands.spawn(Camera2dBundle::default());

    let master_handle: Handle<Image> = asset_server.load("MasterCycleFINAL.png");
    let master_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 8, 5, None, None);
    let master_layout_length = master_layout.textures.len();
    let master_layout_handle = texture_atlases.add(master_layout);

    commands.spawn((
      SpriteBundle {
        texture: master_handle,
        transform: Transform {
            scale: Vec3::splat(2.0),
            ..default()
        },
        ..default()
      },
      TextureAtlas {
        layout: master_layout_handle,
        index: 0,
      },
      AnimationTimer(Timer::from_seconds(ANIMATION_TIME, TimerMode::Repeating)),
      AnimationFrameCount(master_layout_length),
      Velocity::new(),
      Player {
        animation_state: SpriteState::Idle,
        timer: Timer::from_seconds(SpriteState::Idle.animation_speed(), TimerMode::Repeating),

      },
    )
    );


}



fn move_player(
    time: Res<Time>,
    key_pressed: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut player_transform, mut player_velocity) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if key_pressed.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
    }

    if key_pressed.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
    }

    if key_pressed.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
    }

    if key_pressed.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
    }

    let delta_t = time.delta_seconds();
    let acc = ACCELERATION * delta_t;

    player_velocity.v = if deltav.length() > 0. {
        (player_velocity.v + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if player_velocity.v.length() > acc {
        player_velocity.v + (player_velocity.v.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = player_velocity.v * delta_t;

    let new_position = player_transform.translation + Vec3::new(change.x, 0., 0.);
    if new_position.x >= -(LEVEL_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_position.x <= LEVEL_W / 2. - (TILE_SIZE as f32) / 2.
    {
        player_transform.translation = new_position;
    }

    let new_pos = player_transform.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(LEVEL_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= LEVEL_H / 2. - (TILE_SIZE as f32) / 2.
    {
        player_transform.translation = new_pos;
    }
}


fn player_animation(
    time: Res<Time>,
    mut player_query: Query<
        (
            &Velocity,
            &mut TextureAtlas,
            &mut AnimationTimer,
            &AnimationFrameCount,
            &mut Player
        ),
        With<Player>,
    >,
) {
    let (velocity, mut texture_atlas, mut timer, frame_count, mut player) = player_query.single_mut();
        let new_state = if velocity.v.cmpeq(Vec2::ZERO).all() {
            SpriteState::Idle
        } else if velocity.v.x < 0.{
            SpriteState::LeftRun         
        } else if velocity.v.x > 0. {
            SpriteState::RightRun
        } else if velocity.v.y < 0.{ 
            SpriteState::ForwardRun
        } else if velocity.v.y > 0. {
            SpriteState::BackwardRun
        } else {
            SpriteState::Idle
        };

        if new_state != player.animation_state {
            player.animation_state = new_state;
            player.timer = Timer::from_seconds(
                player.animation_state.animation_speed(),
                TimerMode::Repeating,
            );
            let start = player.animation_state.animation_indices();
            texture_atlas.index = start.start;
        }

        player.timer.tick(time.delta());

        if player.timer.just_finished() {
            let indices = player.animation_state.animation_indices();
            texture_atlas.index = if texture_atlas.index + 1 >= indices.end {
                indices.start
            } else {
                texture_atlas.index + 1
            };

        print!("state: {}\n", texture_atlas.index);

        }

}