use bevy::prelude::*;

#[derive(Resource)]
pub struct Wind {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct CountdownTimer {
    pub timer: Timer,
}
