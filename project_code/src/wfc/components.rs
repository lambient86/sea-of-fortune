use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Wall,
    Ground,
    Hole,
    Void
}

#[derive(Resource)]
pub struct DungeonTileSheet(pub Handle<Image>, pub Handle<TextureAtlasLayout>);

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
        weights.insert(TileType::Wall, 0.25);   // 25% chance for walls
        weights.insert(TileType::Ground, 0.5);  // 50% chance for ground
        weights.insert(TileType::Hole, 0.05);    // 5% chance for holes
        weights.insert(TileType::Void, 0.2);     // 20% chance for void
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

pub const VALID_NEIGHBORS: [(TileType, TileType); 16] = [
    // Valid horizontal/vertical neighbors
    (TileType::Wall, TileType::Wall),
    (TileType::Wall, TileType::Ground),
    (TileType::Ground, TileType::Wall),
    (TileType::Ground, TileType::Ground),
    (TileType::Ground, TileType::Hole),
    (TileType::Hole, TileType::Ground),
    (TileType::Void, TileType::Void),
    (TileType::Void, TileType::Wall),

    // Valid diagonal neighbors
    (TileType::Wall, TileType::Ground),
    (TileType::Ground, TileType::Wall),
    (TileType::Ground, TileType::Ground),
    (TileType::Ground, TileType::Hole),
    (TileType::Hole, TileType::Ground),
    (TileType::Hole, TileType::Hole),
    (TileType::Void, TileType::Void),
    (TileType::Void, TileType::Wall),
];
