use bevy::prelude::*;

//constants
pub const SKELETON_ANIMATION_TIME: f32 = 0.4;
pub const SKELETON_PROJECTILE_LIFETIME: f32 = 2.;
pub const SKELETON_PROJECTILE_SPEED: f32 = 300.;

//SKELETON base stats
pub const SKELETON_MAX_HP: f32 = 6.;
pub const SKELETON_ATTACK_DIST: f32 = 400.;
pub const SKELETON_MOVEMENT_SPEED: f32 = 100.;
pub const SKELETON_AGRO_STOP: f32 = 100.;
pub const SKELETON_AGRO_RANGE: f32 = 600.;

/// Struct to represent the skeleton entity
#[derive(Component)]
pub struct Skeleton {
    pub rotation_speed: f32,
    pub current_hp: f32,
    pub max_hp: f32,
}

#[derive(Component)]
pub struct SkeletonProjectile {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Lifetime(pub f32);
