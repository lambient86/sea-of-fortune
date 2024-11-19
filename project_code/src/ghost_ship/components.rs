use bevy::prelude::*;

//constants
pub const GHOSTSHIP_ANIMATION_TIME: f32 = 0.25;
pub const GHOSTSHIP_PROJECTILE_LIFETIME: f32 = 5.;
pub const GHOSTSHIP_PROJECTILE_SPEED: f32 = 175.;

//Bat base stats
pub const GHOSTSHIP_MAX_HP: f32 = 2.;
pub const GHOSTSHIP_ATTACK_DIST: f32 = 800.;
pub const GHOSTSHIP_MOVEMENT_SPEED: f32 = 150.;
pub const GHOSTSHIP_AGRO_STOP: f32 = 300.;
pub const GHOSTSHIP_AGRO_RANGE: f32 = 1000.;

/// Struct to represent the bat entity
#[derive(Component)]
pub struct GhostShip {
    pub rotation_speed: f32,
    pub current_hp: f32,
    pub max_hp: f32,
}

#[derive(Component)]
pub struct GhostShipProjectile;

#[derive(Component)]
pub struct Lifetime(pub f32);
