use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::components::GameState;
use crate::hud::systems::init_player_hud;
use crate::GameworldState;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// Builds the player plugin
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameworldState::Island),
            (
                spawn_player,
                spawn_weapon.after(spawn_player),
                init_player_hud.after(spawn_player),
            ),
        )
        .add_systems(
            OnEnter(GameworldState::Dungeon),
            (spawn_player, spawn_weapon.after(spawn_player)),
        )
        .add_systems(
            Update,
            (
                move_player,
                player_animation.after(move_player),
                sword_attack,
                musket_attack,
                check_player_health,
                musketball_lifetime_check,
                move_musketball,
                move_weapon.after(move_player),
                swap_weapon,
            )
                .run_if(in_state(GameworldState::Island).or_else(in_state(GameworldState::Dungeon)))
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            OnExit(GameworldState::Island),
            (despawn_player, despawn_weapon, despawn_musketballs),
        )
        .add_systems(
            OnExit(GameworldState::Dungeon),
            (despawn_player, despawn_weapon, despawn_musketballs),
        );
    }
}
