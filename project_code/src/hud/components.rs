#[derive(Component)]
struct PlayerHP;

#[derive(Component)]
struct ShipHP;

#[derive(Resource)]
struct ShipStats {
    hp: f32,
    gold: i32,
    wind: Direction,
}

#[derive(Resource)]
struct PlayerStats {
    hp: f32,
    gold: i32,
}

#[derive(Component)]
enum Direction {
    NORTH,
    NORTHWEST,
    WEST,
    SOUTHWEST,
    SOUTH,
    SOUTHEAST,
    EAST,
    NORTHEAST,
}
