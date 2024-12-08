use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::{PlayerHUD, ShipHPText, ShipStats};
use systems::*;

use crate::{
    boat::systems::spawn_boat, components::GameworldState, level::systems::despawn_with,
    player::systems::spawn_player,
};

pub struct HUDPlugin;

// impl Plugin for HUDPlugin {
//     /// Builds the boat plugin
//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             OnEnter(GameworldState::Ocean),
//             (init_ship_hud).after(spawn_boat),
//         )
//         .add_systems(OnExit(GameworldState::Ocean), despawn_with::<ShipHPText>)
//         .add_systems(
//             OnEnter(GameworldState::Island),
//             (init_player_hud).after(spawn_player),
//         );
//     }
// }

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameworldState::Island), init_player_hud)
            .add_systems(OnEnter(GameworldState::Dungeon), init_player_hud)
            .add_systems(
                Update,
                update_player_hud.run_if(
                    in_state(GameworldState::Island).or_else(in_state(GameworldState::Dungeon)),
                ),
            )
            .add_systems(OnExit(GameworldState::Island), despawn_with::<PlayerHUD>)
            .add_systems(OnExit(GameworldState::Dungeon), despawn_with::<PlayerHUD>);
    }
}
