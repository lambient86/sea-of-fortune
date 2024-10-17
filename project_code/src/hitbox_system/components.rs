use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

// Hitbox component: Represents an area that can cause interactions
#[derive(Component)]
pub struct Hitbox {
    pub size: Vec2,
    pub offset: Vec2,
    pub lifetime: Option<Timer>,
}

// Hurtbox component: Represents an area that can receive interactions
#[derive(Component)]
pub struct Hurtbox {
    pub size: Vec2,
    pub offset: Vec2,
}

// Colliding component: Added to entities when a collision is detected
#[derive(Component)]
pub struct Colliding(pub i32);
