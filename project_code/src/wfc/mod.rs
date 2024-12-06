pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;

use crate::components::{Background, GameworldState};
use crate::player::components::Sword;
use crate::level::components::*;
use crate::level::systems::*;
use crate::wfc::systems::*;

pub struct WFCPlugin;

impl Plugin for WFCPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileWeights>()
        .add_systems(Startup, load_dungeon)
        .add_systems(OnEnter(
            GameworldState::Dungeon), 
            (setup_wfc, despawn_with::<Background>)
        )
        .add_systems(OnExit(
            GameworldState::Dungeon),
           (despawn_with::<Tile>)
        );
    }
}