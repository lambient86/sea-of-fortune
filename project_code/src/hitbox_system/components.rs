use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

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
