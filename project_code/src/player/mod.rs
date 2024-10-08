use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// Builds the player plugin
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, player_animation.after(move_player))
            .add_systems(Update, player_attack)
            .add_systems(Update, check_player_health);
    }
}
