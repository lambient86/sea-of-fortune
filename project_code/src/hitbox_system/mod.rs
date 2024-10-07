use bevy::prelude::*;
mod components;
mod systems;

pub use components::*;
pub use systems::*;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            check_hitbox_collisions,
            check_hurtbox_collisions,
        ));
    }
}