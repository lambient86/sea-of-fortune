use bevy::prelude::*;

pub const CANNONBALL_SPEED: f32 = 500.;
pub const CANNONBALL_LIFETIME: f32 = 6.;
pub const MAX_ACCEL: f32 = 800.;

/// Struct to represent the boat entity that players will be represented as
/// in the ocean world
#[derive(Component)]
pub struct Boat {
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub acceleration: f32,
}

/// Struct to represent the cannon ball being shot by the player controlled
/// boat
#[derive(Component)]
pub struct Cannonball;

/// Struct representation for cannonball projectile lifetime
#[derive(Component)]
pub struct Lifetime(pub f32);

/// Struct to maintain last boat position for out of transition spawning
pub struct BoatLastPosition {
    pub last_pos: Vec2,
}

/// Velocity struct
#[derive(Component)]
pub struct Velocity {
    pub v: Vec2,
}

/// Velocity implementation
impl Velocity {
    pub fn new() -> Self {
        Self {
            //sets x and y velocity dsto 0
            v: Vec2::splat(0.),
        }
    }
}