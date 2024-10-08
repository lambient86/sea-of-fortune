use bevy::prelude::*;

//constants
pub const ATTACK_DIST: f32 = 200.;
pub const ANIMATION_TIME: f32 = 0.2;

/// Struct to represent the bat entity
#[derive(Component)]
pub struct Bat {
    pub rotation_speed: f32,
    pub current_hp: f32,
    pub max_hp: f32,
}

/// Struct to represent the rotation of the bat to the player
#[derive(Component)]
pub struct RotateToPlayer {
    pub rotation_speed: f32,
}

/// Struct for the time between the bat's animation frames
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    /// Initializes the animation timer
    pub fn new(timer: Timer) -> AnimationTimer {
        AnimationTimer(timer)
    }
}

/// Struct for the count of frames in the bats animation
#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

impl AnimationFrameCount {
    /// Initializes the animation frame count
    pub fn new(size: usize) -> AnimationFrameCount {
        AnimationFrameCount(size)
    }
}

/// Struct to represent a bat entities velocity
#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

impl Velocity {
    /// Initializes a new velocity struct for a bat entity
    pub fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}
