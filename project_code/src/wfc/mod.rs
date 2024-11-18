pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;

pub struct WFCPlugin;

impl Plugin for WFCPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileWeights>()
           .add_systems(Startup, setup_wfc);
    }
}