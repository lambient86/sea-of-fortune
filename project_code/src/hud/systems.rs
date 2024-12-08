use bevy::prelude::*;

pub fn init_hud(
    mut commands: Commmands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, With<Player>)>,
    mut inventory_query: Query<(&mut Inventory, With<Player>)>,
    mut wind_query: Query<(&mut Wind, With<Wind>)>,
) {
    let player = player_query.single();
    let inventory = inventory_query.single();
    let _wind = wind_query.single();

    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(PlayerStats {
        hp: player.hp,
        gold: inventory.money,
    });

    commands.insert_resource(ShipStats {
        hp: player.hp,
        gold: inventory.money,
        wind: _wind.direction,
    });
}
