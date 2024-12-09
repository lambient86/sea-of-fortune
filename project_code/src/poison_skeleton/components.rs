use bevy::prelude::*;

//constants
pub const PSKELETON_ANIMATION_TIME: f32 = 0.4;
pub const PSKELETON_PROJECTILE_LIFETIME: f32 = 2.;
pub const PSKELETON_PROJECTILE_SPEED: f32 = 300.;

//SKELETON base stats
pub const PSKELETON_MAX_HP: f32 = 6.;
pub const PSKELETON_ATTACK_DIST: f32 = 400.;
pub const PSKELETON_MOVEMENT_SPEED: f32 = 100.;
pub const PSKELETON_AGRO_STOP: f32 = 100.;
pub const PSKELETON_AGRO_RANGE: f32 = 600.;

/// Struct to represent the skeleton entity
#[derive(Component)]
pub struct PoisonSkeleton {
    pub rotation_speed: f32,
    pub current_hp: f32,
    pub max_hp: f32,
}

#[derive(Component)]
pub struct PSkeletonProjectile {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Lifetime(pub f32);
