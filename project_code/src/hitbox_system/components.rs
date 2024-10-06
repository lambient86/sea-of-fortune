use bevy::prelude::*;
use bevy::math::bounding::Aabb2d;

#[derive(Component)]
pub struct Hitbox {
    pub aabb: Aabb2d,
}

#[derive(Component)]
pub struct Hurtbox {
    pub aabb: Aabb2d,
}

#[derive(Component)]
pub struct Colliding;