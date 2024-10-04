use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "bv05 Better Motion";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SIZE: f32 = 32.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 3600.;


#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
}


#[derive(Component)]
struct AttackCooldown {
    remaining: f32,
}

#[derive(Component)]
struct LastDirection {
    direction: Vec2,
}

// Last direction detected by user for attack fn
impl LastDirection {
    fn new() -> Self {
        Self {
            direction: Vec2::ZERO,
        }
    }
}

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
        // Added the attack system
        .add_systems(Update, player_attack)
        .run();
}

pub fn setup(mut commands: Commands) {
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
        // Cooldown and last direction property added to player on startup
        .insert(AttackCooldown { remaining: 0.0 })
        .insert(LastDirection::new())
        .insert(Player);  
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut LastDirection), With<Player>>,
) {
    let (mut pt, mut pv, mut ld) = player.single_mut();

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

    // Keep track of last direction input for attack
    if deltav.length() > 0. {
        ld.direction = deltav.normalize();
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

// attack function 
pub fn player_attack(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &LastDirection, &mut AttackCooldown), With<Player>>,
) {
    for (_, last_direction, mut cooldown) in player_query.iter_mut() {
        if cooldown.remaining > 0.0 {
            cooldown.remaining -= time.delta_seconds();
        }

        if input.pressed(KeyCode::Space) && cooldown.remaining <= 0.0 {
            // Add logic for the attack here, projectiles, damage, etc
            println!("Player attacked in direction: {:?}", last_direction.direction);

            cooldown.remaining = 1.0;
        }
    }
}


