use bevy::prelude::*;

//constants
pub const ROCK_ANIMATION_TIME: f32 = 0.5;

//Rock base stats
pub const ROCK_MAX_HP: f32 = 15.;
pub const ROCK_MOVEMENT_SPEED: f32 = 50.;
pub const ROCK_AGRO_RANGE: f32 = 10000.;

/// Struct to represent the bat entity
#[derive(Component)]
pub struct Rock {
    pub current_hp: f32,
}
