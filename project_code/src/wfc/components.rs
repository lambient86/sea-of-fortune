use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Wall,
    Ground,
    Hole,
}

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Resource)]
pub struct TileWeights {
    pub weights: HashMap<TileType, f32>,
}

impl Default for TileWeights {
    fn default() -> Self {
        let mut weights = HashMap::new();
        weights.insert(TileType::Wall, 0.3);   // 30% chance for walls
        weights.insert(TileType::Ground, 0.6);  // 60% chance for ground
        weights.insert(TileType::Hole, 0.1);    // 10% chance for holes
        Self { weights }
    }
}

#[derive(Resource)]
pub struct WFCState {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Option<TileType>>>,
    pub entropy: Vec<Vec<Vec<(TileType, f32)>>>,
}

pub const VALID_NEIGHBORS: [(TileType, TileType); 12] = [
    // Valid horizontal/vertical neighbors
    (TileType::Wall, TileType::Wall),
    (TileType::Wall, TileType::Ground),
    (TileType::Ground, TileType::Wall),
    (TileType::Ground, TileType::Ground),
    (TileType::Ground, TileType::Hole),
    (TileType::Hole, TileType::Ground),
    // Valid diagonal neighbors
    (TileType::Wall, TileType::Ground),
    (TileType::Ground, TileType::Wall),
    (TileType::Ground, TileType::Ground),
    (TileType::Ground, TileType::Hole),
    (TileType::Hole, TileType::Ground),
    (TileType::Hole, TileType::Hole),
];