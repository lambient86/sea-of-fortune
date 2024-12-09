use bevy::prelude::*;

pub const STORM_DAMAGE_INTERVAL: f32 = 5.0;

#[derive(Component)]
pub struct Storm {
    pub damage_timer: Timer,
}
