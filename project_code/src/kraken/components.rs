use bevy::prelude::*;

//constants
pub const KRAKEN_ANIMATION_TIME: f32 = 0.25;
pub const KRAKEN_PROJECTILE_LIFETIME: f32 = 5.;
pub const KRAKEN_PROJECTILE_SPEED: f32 = 175.;
pub const KRAKEN_LIFETIME: f32 = 120.;

//Bat base stats
pub const KRAKEN_MAX_HP: f32 = 2.;
pub const KRAKEN_ATTACK_DIST: f32 = 800.;
pub const KRAKEN_MOVEMENT_SPEED: f32 = 150.;
pub const KRAKEN_AGRO_STOP: f32 = 300.;
pub const KRAKEN_AGRO_RANGE: f32 = 1000.;

/// Struct to represent the bat entity
#[derive(Component)]
pub struct Kraken {
    pub rotation_speed: f32,
    pub current_hp: f32,
    pub max_hp: f32,
}

#[derive(Component)]
pub struct KrakenProjectile;

#[derive(Component)]
pub struct Lifetime(pub f32);
