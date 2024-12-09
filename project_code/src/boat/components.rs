use bevy::prelude::*;

use crate::components::BoundingBox;

pub const CANNONBALL_SPEED: f32 = 800.;
pub const CANNONBALL_LIFETIME: f32 = 6.;
pub const MAX_ACCEL: f32 = 800.;
pub const BOAT_MAX_HP: f32 = 5.;

/// Struct to represent the boat entity that players will be represented as
/// in the ocean world
#[derive(Component)]
pub struct Boat {
    pub id: i32,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub acceleration: f32,
    pub aabb: BoundingBox,
    pub health: f32,
    pub max_health: f32,
    pub cannon_damage: f32,
}

/// Struct to represent the cannon ball being shot by the player controlled
/// boat
#[derive(Component)]
pub struct Cannonball;

/// Struct to maintain last boat position for out of transition spawning
pub struct BoatLastPosition {
    pub last_pos: Vec2,
}

/// Cannonball velocity struct
#[derive(Component)]
pub struct CannonballVelocity {
    pub v: Vec3,
}
