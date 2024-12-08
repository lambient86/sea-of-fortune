use bevy::prelude::*;

pub const WHIRLPOOL_HP: f32 = 2.;
pub const WHIRLPOOL_LIFETIME: f32 = 120.;

#[derive(Component)]
pub struct Whirlpool {
    pub rotation_speed: f32,
    pub current_hp: f32,
    pub max_hp: f32,
}
