use bevy::prelude::*;

use crate::{player::components::Player, wind::components::Wind};

use super::components::*;

// pub fn init_player_hud(
//     mut commands: Commands,
//     // player_query: Query<&mut Player, With<Player>>,
//     asset_server: Res<AssetServer>,
// ) {
//     commands.insert_resource(PlayerStats {
//         hp: player.health,
//         gold: player.inventory.money,
//     });

//     // money and hp container
//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 display: Display::Flex,
//                 flex_direction: FlexDirection::Row,
//                 justify_content: JustifyContent::SpaceBetween,
//                 padding: UiRect::all(Val::Px(20.0)),
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 ..default()
//             },
//             ..default()
//         })
//         .with_children(|parent| {
//             parent
//                 .spawn(NodeBundle {
//                     style: Style {
//                         flex_direction: FlexDirection::Column,
//                         ..default()
//                     },
//                     ..default()
//                 })
//                 .with_children(|stats_parent| {
//                     stats_parent.spawn((
//                         TextBundle::from_section(
//                             format!("Health: {}", player.health),
//                             TextStyle {
//                                 font: asset_server.load("assets/pixel_pirate.ttf"),
//                                 font_size: 200.0,
//                                 color: Color::srgb(242.0, 231.0, 218.0),
//                             },
//                         ),
//                         PlayerHPText,
//                     ));
//                     stats_parent.spawn((
//                         TextBundle::from_section(
//                             format!("Gold: {}", player.inventory.money),
//                             TextStyle {
//                                 font: asset_server.load("assets/pixel_pirate.ttf"),
//                                 font_size: 200.0,
//                                 color: Color::srgb(242.0, 231.0, 218.0),
//                             },
//                         ),
//                         GoldText,
//                     ));
//                 });
//         });
// }

pub fn init_player_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            PlayerHUD,
        ))
        .with_children(|parent| {
            // Health Display
            parent.spawn((
                TextBundle::from_section(
                    "Health: 3/3",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(242.0, 231.0, 218.0),
                        ..default()
                    },
                ),
                PlayerHPText,
            ));

            // Gold Display
            parent.spawn((
                TextBundle::from_section(
                    "Gold: 0",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(242.0, 231.0, 218.0),
                        ..default()
                    },
                ),
                GoldText,
            ));
        });
}

pub fn init_ship_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _wind: Option<Res<Wind>>,
) {
    commands.insert_resource(ShipStats {
        hp: 100.0,
        gold: 50,
        wind: CardinalDirection::EAST,
    });

    // money and hp container
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(20.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|stats_parent| {
                    stats_parent.spawn((
                        TextBundle::from_section(
                            format!("Health: {}", 100.),
                            TextStyle {
                                font: asset_server.load("assets/pixel_pirate.ttf"),
                                font_size: 32.0,
                                color: Color::srgb(242.0, 231.0, 218.0),
                            },
                        ),
                        PlayerHPText,
                    ));
                    stats_parent.spawn((
                        TextBundle::from_section(
                            format!("Gold: {}", 50.0),
                            TextStyle {
                                font: asset_server.load("assets/pixel_pirate.ttf"),
                                font_size: 32.0,
                                color: Color::srgb(242.0, 231.0, 218.0),
                            },
                        ),
                        PlayerHPText,
                    ));
                });
        });
}

pub fn update_player_hud(
    player_query: Query<(&Player)>,
    mut text_query: Query<(&mut Text, Option<&PlayerHPText>, Option<&GoldText>)>,
) {
    if let Ok(player) = player_query.get_single() {
        for (mut text, health, gold) in text_query.iter_mut() {
            if health.is_some() {
                text.sections[0].value = format!("Health: {}/{}", player.health, player.max_health);
            } else if gold.is_some() {
                text.sections[0].value = format!("Gold: {}", player.inventory.money);
            }
        }
    }
}
