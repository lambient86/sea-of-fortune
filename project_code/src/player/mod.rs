use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod controls;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update,move_player)
            .add_systems(Update, player_animation.after(move_player));
    }
}