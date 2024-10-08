use bevy::prelude::*;
mod components;
mod systems;

pub use components::*;
pub use systems::*;

// Plugin to set up the hitbox and hurtbox system
pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            check_hitbox_hurtbox_collisions,
            update_hitbox_lifetimes,  // Add the new system
            draw_debug_boxes,
        ));
    }
}