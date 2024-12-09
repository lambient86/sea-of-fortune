use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
use crate::GameworldState;
use crate::components::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// Builds the player plugin
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Island), (
                spawn_player,
                spawn_weapon.after(spawn_player),))  
            .add_systems(OnEnter(GameworldState::Dungeon), (
                spawn_player,
                spawn_weapon.after(spawn_player),))
            .add_systems(Update, (    
                move_player.after(spawn_player),
                player_animation.after(move_player),
                sword_attack,
                sword_swoosh_animation,
                musket_attack,
                dagger_attack,
                pistol_attack,
                check_player_health,
                musketball_lifetime_check,
                move_musketball,
                move_weapon.after(move_player),
                swap_weapon,
                )
                .run_if(in_state(GameworldState::Island).or_else(in_state(GameworldState::Dungeon)))
                .run_if(in_state(GameState::Running)))
            .add_systems(OnExit(GameworldState::Island), (
                despawn_player,
                despawn_musketballs,))
            .add_systems(OnExit(GameworldState::Dungeon), (
                despawn_player,
                despawn_musketballs,));
    }
}
