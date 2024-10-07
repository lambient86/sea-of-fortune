use bevy::prelude::*;

/// enum for different types of player input
pub enum PlayerControl {
    Up,     //W
    Down,   //S
    Left,   //A
    Right,  //D
    Attack, //Spacebar, change to Left Mouse Button
    //Charge, //subject to change, for range (right click)
}

/// Player control implementation
impl PlayerControl {

    /// Checks if the expected control was pressed,
    /// * returns true if pressed
    /// else
    /// * returns false
    pub fn pressed(&self, input: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            PlayerControl::Up => {
                input.pressed(KeyCode::KeyW)
            }
            PlayerControl::Down => {
                input.pressed(KeyCode::KeyS)
            }
            PlayerControl::Left => {
                input.pressed(KeyCode::KeyA)
            }
            PlayerControl::Right => {
                input.pressed(KeyCode::KeyD)
            }
            PlayerControl::Attack => {
                input.pressed(KeyCode::Space)
            }
        }
    }
}

/// Checks if a certain input was inputted by the controller. If
/// input was pressed:
/// * returns 1.0
/// else,
/// * returns 0.0
/// 
/// These numbers can be used as a multiplier for movement, or as a check
/// for actions such as attack
pub fn get_player_input(
    control: PlayerControl, 
    input: &Res<ButtonInput<KeyCode>>
) -> f32 {
    //checking for input
    if control.pressed(input) {
        //if given input was found, returns 1.0
        //which can be used as a multiplier for
        //movement or a boolean for an attack
        1.0
    } else {
        0.0
    }
}