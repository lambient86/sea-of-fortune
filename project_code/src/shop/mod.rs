pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;
use components::*;
use crate::components::GameState;
use crate::components::GameworldState; //FOR DEBUG DEMO PURPOSES

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Shop::default())
            .insert_resource(ShopPage::default())
            .add_event::<ShopEvent>()
            .add_systems(Update, (
                handle_shop_events,
                update_shop_ui,
                handle_button_interactions,
                update_sword_damage,
            ).run_if(in_state(GameState::InShop)))
            .add_systems(OnEnter(GameState::InShop), setup_shop_ui)
            .add_systems(OnEnter(GameworldState::Ocean), cleanup_shop_ui) //FOR DEBUG DEMO PURPOSES
            .add_systems(OnExit(GameState::InShop), cleanup_shop_ui);
    }
}