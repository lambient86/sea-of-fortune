use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
use crate::GameworldState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// Builds the player plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Island),spawn_player)
            .add_systems(Update, (    
                move_player,
                player_animation.after(move_player),
                player_attack,
                check_player_health,
                )
                .run_if(in_state(GameworldState::Island)))
            .add_systems(OnExit(GameworldState::Island), despawn_player);
    }
}
