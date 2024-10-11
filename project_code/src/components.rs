use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

/*   GAMEWORLD STATES   */
/// Enum to represent the different states the gameworld can be in. Depending on the state of the gameeworld,
/// different enemies may spawn, the player controls a different entity (boat or player), etc. These states include
/// * Island
/// * Ocean
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameworldState {
    Island,  //the state the gameworld will be in when exploring islands
    Ocean,   //the state the gameworld will be in when exploring the ocean
}