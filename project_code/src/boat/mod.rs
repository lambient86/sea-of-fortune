use bevy::prelude::*;

mod components;
mod systems;
use systems::*;

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    /// Builds the boat plugin
    fn build(&self, app: &mut App) {
            app
                .add_systems(Update,move_boat)
                .add_systems(Update,spawn_boat);
    }
}