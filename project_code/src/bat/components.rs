use bevy::prelude::*;
use crate::shop::components::Item;

//constants
pub const BAT_ANIMATION_TIME: f32 = 0.2;
pub const BAT_PROJECTILE_LIFETIME: f32 = 3.;
pub const BAT_PROJECTILE_SPEED: f32 = 500.;

//Bat base stats
pub const BAT_MAX_HP: f32 = 2.;
pub const BAT_ATTACK_DIST: f32 = 500.;
pub const BAT_MOVEMENT_SPEED: f32 = 200.;
pub const BAT_AGRO_STOP: f32 = 150.;
pub const BAT_AGRO_RANGE: f32 = 700.;

/// Struct to represent the bat entity
#[derive(Component)]
pub struct Bat {
    pub rotation_speed: f32,
    pub current_hp: f32,
    pub max_hp: f32,
}

#[derive(Component)]
pub struct BatProjectile;

#[derive(Component)]
pub struct Lifetime(pub f32);

#[derive(Component)]
pub struct Loot;

#[derive(Bundle)]
pub struct LootBundle {
    pub item: Item,
    pub sprite_bundle: SpriteBundle,
    pub marker: Loot,
}
