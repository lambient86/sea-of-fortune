use bevy::{math::bounding::Aabb2d, prelude::*};
use crate::level::components::IslandType;
use crate::shop::components::*;

#[derive(Component)]
pub struct Background;

#[derive(Resource)]
pub struct StateTransitionCooldown {
    pub timer: Timer,
}

impl Default for StateTransitionCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct TransitionImmunity {
    pub timer: Timer,
}


/*   GAMEWORLD STATES   */
/// Separate from GameState, GameworldState is an enum to represent the different states the gameworld can be in. Depending
/// on the state of the gameworld, different enemies may spawn, the player controls a different entity (boat or player), etc.
/// These states include
/// * MainMenu
/// * Island
/// * Ocean
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameworldState {
    MainMenu, //the state the gameworld will be in when in the main menu
    Island,   //the state the gameworld will be in when exploring islands
    Ocean,    //the state the gameworld will be in when exploring the ocean
    Dungeon,  //the state the gameworld will be in when exploring dungeons
}

/*   GAME STATES   */
/// Separate from GameworldState, GameState is an enum that represents the state that the game itself is in. Depending on the
/// state of the game, actions may pause with the intention of being resumed later, such as making the world static while in the shop
/// These states include
/// * Running
/// * InShop
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Running, //the state the game is in while it is running
    InShop,  //the state the game is in while the player is in the shop
}

#[derive(Component)]
pub struct BoundingBox {
    pub aabb: Aabb2d,
}

impl BoundingBox {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        BoundingBox {
            aabb: Aabb2d::new(position, size),
        }
    }

    pub fn update_position(&mut self, new_position: Vec2) {
        let half_size = (self.aabb.max - self.aabb.min) / 2.0;
        self.aabb = Aabb2d::new(new_position, half_size);
    }
}

#[derive(Resource)]
pub struct SpawnLocations {
    pub player: Vec3,
    pub door: Vec3,
}

impl Default for SpawnLocations {
    fn default() -> Self {
        Self {
            player: Vec3::ZERO,
            door: Vec3::ZERO,
        }
    }
}

// GLOBAL PLAYER ENTITIES
#[derive(Resource)]
pub struct PlayerEntities {
    pub players: Vec<Entity>,
    pub next_id: u8,
    pub current_entity: Option<Entity>,
    pub saved_inventory: Option<Inventory>,  // Add this
}

impl Default for PlayerEntities {
    fn default() -> Self {
        Self {
            players: Vec::with_capacity(4),
            next_id: 1,
            current_entity: None,
            saved_inventory: None,
        }
    }
}

#[derive(Resource)]
pub struct CurrentIslandType {
    pub island_type: IslandType,
}

impl Default for CurrentIslandType {
    fn default() -> Self {
        Self {
            island_type: IslandType::Level1
        }
    }
}
