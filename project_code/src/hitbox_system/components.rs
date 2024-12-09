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

    pub dmg: f32,
}

// Hurtbox component: Represents an area that can receive interactions
#[derive(Component)]
pub struct Hurtbox {
    pub size: Vec2,
    pub offset: Vec2,
    pub colliding: Collision,

    pub iframe: Timer,
    pub entity: i32,
    pub enemy: bool,
}

pub struct Collision {
    pub is: bool,
    pub dmg: f32,
}

impl Collision {
    pub fn default() -> Collision {
        Collision { is: false, dmg: 0. }
    }
}
