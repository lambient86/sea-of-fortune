use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

/*   GAMEWORLD STATES   */
/// Separate from GameState, GameworldState is an enum to represent the different states the gameworld can be in. Depending 
/// on the state of the gameworld, different enemies may spawn, the player controls a different entity (boat or player), etc. 
/// These states include
/// * MainMenu
/// * Island
/// * Ocean
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameworldState {
    MainMenu,   //the state the gameworld will be in when in the main menu
    Island,     //the state the gameworld will be in when exploring islands
    Ocean,      //the state the gameworld will be in when exploring the ocean
    Dungeon,    //the state the gameworld will be in when exploring dungeons
}

/*   GAME STATES   */
/// Separate from GameworldState, GameState is an enum that represents the state that the game itself is in. Depending on the
/// state of the game, actions may pause with the intention of being resumed later, such as making the world static while in the shop
/// These states include
/// * Running
/// * InShop
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Running,    //the state the game is in while it is running
    InShop,     //the state the game is in while the player is in the shop
}