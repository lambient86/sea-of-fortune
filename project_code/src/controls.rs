use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// enum for different types of player input
pub enum PlayerControl {
    Up,         //W
    Down,       //S
    Left,       //A
    Right,      //D
    Attack,     //Left Mouse Button
    Secondary,  //Right Mouse Button
}

/// Struct to represent current mouse position
#[derive(Resource, Default)]
pub struct CurrMousePos(pub Vec2);

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

/*  UPDATE_MOUSE_POS FUNCTION   */
pub fn update_mouse_pos(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut mouse_pos: ResMut<CurrMousePos>,
) {
    //getting camera info and transform (works assuming one main camera and main entity)
    let (camera, camera_transform) = q_camera.single();

    //getting window information (works assuming one main window)
    let window = q_window.single();

    //checking that cursor is in window and getting it's position
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        {
            mouse_pos.0 = world_position;
        }
}
