use crate::wind::components::Wind;
use bevy::prelude::*;

pub fn init_wind(mut commands: Commands) {
    commands.insert_resource(Wind {
        direction: Vec2::new(0.0, 90.0),
    });
}

pub fn change_wind_dir(wind: ResMut<Wind>) {}
