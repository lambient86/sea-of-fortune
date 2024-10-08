use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

pub struct BatPlugin;

impl Plugin for BatPlugin {
    /// Builds the bat plugin
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bat)
            .add_systems(Update, animate_bat)
            .add_systems(Update, rotate_bat)
            .add_systems(Update, bat_attack)
            .add_systems(Update, bat_damaged);
    }
}
