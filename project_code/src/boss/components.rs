use bevy::prelude::*;

//constants
pub const BOSS_ANIMATION_TIME: f32 = 0.5;

//Boss base stats
pub const BOSS_MAX_HP: f32 = 50.; // Much higher HP than rock
pub const BOSS_MOVEMENT_SPEED: f32 = 75.; // Slightly faster than rock
pub const BOSS_AGRO_RANGE: f32 = 10000.;

/// Struct to represent the boss entity
#[derive(Component)]
pub struct Boss {
    pub current_hp: f32,
}