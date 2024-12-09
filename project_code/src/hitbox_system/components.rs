use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

//ENTITIES
pub const PLAYER: i32 = 0;
pub const BOAT: i32 = 1;
pub const BAT: i32 = 2;
pub const KRAKEN: i32 = 3;
pub const GHOSTSHIP: i32 = 4;
pub const ROCK: i32 = 5;
pub const SKELETON: i32 = 6;
pub const SKEL2: i32 = 7;
pub const WHIRLPOOL: i32 = 8;
pub const BOSS: i32 = 9;
pub const PSKELETON: i32 = 10;

// Hitbox component: Represents an area that can cause interactions
#[derive(Component)]
pub struct Hitbox {
    pub size: Vec2,
    pub offset: Vec2,
    pub lifetime: Option<Timer>,
    pub projectile: bool,

    pub entity: i32,
    pub enemy: bool,
}

// Hurtbox component: Represents an area that can receive interactions
#[derive(Component)]
pub struct Hurtbox {
    pub size: Vec2,
    pub offset: Vec2,
    pub colliding: bool,

    pub iframe: Timer,
    pub entity: i32,
    pub enemy: bool,
}
