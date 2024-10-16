use bevy::prelude::*;

//constants
pub const BAT_ANIMATION_TIME: f32 = 0.2;
pub const BAT_PROJECTILE_LIFETIME: f32 = 3.;
pub const BAT_PROJECTILE_SPEED: f32 = 500.;

//Bat base stats
pub const BAT_MAX_HP: f32 = 1.;
pub const BAT_ATTACK_DIST: f32 = 500.;
pub const BAT_MOVEMENT_SPEED: f32 = 200.;
pub const BAT_AGRO_STOP_RADIUS: f32 = 150.;
pub const BAT_AGRO_RANGE: f32 = 700.;

/// Struct to represent the bat entity
#[derive(Component)]
pub struct Bat {
    pub rotation_speed: f32,
    pub current_hp: f32,
    pub max_hp: f32,
    pub velocity: Velocity,
}

#[derive(Component)]
pub struct BatProjectile;

#[derive(Component)]
pub struct Lifetime(pub f32);

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
    pub v: Vec3,
}

impl Velocity {
    /// Initializes a new velocity struct for a bat entity
    pub fn new() -> Self {
        Self { v: Vec3::splat(0.) }
    }
}
