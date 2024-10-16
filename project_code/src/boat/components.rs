use bevy::prelude::*;

/// Struct to represent the boat entity that players will be represented as
/// in the ocean world
#[derive(Component)]
pub struct Boat {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

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