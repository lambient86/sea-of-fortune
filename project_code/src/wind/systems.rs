use crate::wind::components::Wind;
use bevy::prelude::*;
use rand::Rng;

pub fn init_wind(mut commands: Commands) {
    commands.insert_resource(Wind {
        direction: Vec2::new(0.0, 90.0),
    });
}

pub fn change_wind_dir(mut wind: ResMut<Wind>, mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..=360.0);
    let y = rng.gen_range(0.0..=360.0);
    wind.direction = Vec2::new(x, y);
}
