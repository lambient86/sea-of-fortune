use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::components::GameState;
use crate::{GameworldState, UDP};
use systems::*;
