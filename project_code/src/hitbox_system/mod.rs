use crate::boat::systems::move_boat;
use crate::player::systems::move_player;
use bevy::prelude::*;
pub(crate) mod components;
mod systems;

pub use components::*;
pub use systems::*;

// Plugin to set up the hitbox and hurtbox system
pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_hitbox_hurtbox_collisions,
                update_hitbox_lifetimes, // Add the new system
                                         //draw_debug_boxes.after(move_boat).after(move_player),
            ),
        );
    }
}
