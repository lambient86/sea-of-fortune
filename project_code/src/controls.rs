use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;

/// enum for different types of player input
pub enum PlayerControl {
    Up,         //W
    Down,       //S
    Left,       //A
    Right,      //D
    Interact,  //E 
    Attack,     //Left Mouse Button
    Secondary,  //Right Mouse Button
}

/// Player control implementation
impl PlayerControl {

    /// Checks if the expected control was pressed,
    /// * returns true if pressed
    /// else
    /// * returns false
    pub fn pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>, mouse_input: &Res<ButtonInput<MouseButton>>) -> bool {
        match self {
            PlayerControl::Up => {
                keyboard_input.pressed(KeyCode::KeyW)
            }
            PlayerControl::Down => {
                keyboard_input.pressed(KeyCode::KeyS)
            }
            PlayerControl::Left => {
                keyboard_input.pressed(KeyCode::KeyA)
            }
            PlayerControl::Right => {
                keyboard_input.pressed(KeyCode::KeyD)
            }
            PlayerControl::Interact => {
                keyboard_input.pressed(KeyCode::KeyE)
            }
            PlayerControl::Attack => {
                mouse_input.pressed(MouseButton::Left)
            }
            PlayerControl::Secondary => {
                mouse_input.pressed(MouseButton::Right)
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
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mouse_input: &Res<ButtonInput<MouseButton>>,
) -> f32 {
    //checking for input
    if control.pressed(keyboard_input, mouse_input) {
        //if given input was found, returns 1.0
        //which can be used as a multiplier for
        //movement or a boolean for an attack
        1.0
    } else {
        0.0
    }
}

/*  GET_MOUSE_POSITION FUNCTION   */
// Gets the current (x, y) position of the mouse cursor and returns it
