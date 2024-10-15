use bevy::prelude::*;

/// This is a file that contains the code for transitional boxes in. When collided with by a player,
/// the player initiates a transition between subworlds

/// Struct to represent the transition box
#[derive(Component)]
pub struct TransitionBox {
    pub position: Vec2,
}