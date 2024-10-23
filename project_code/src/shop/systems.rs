use bevy::prelude::*;
use super::components::*;
use crate::player::components::Player;

pub fn setup_shop_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }, ShopUI))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Shop",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));

            // Buttons
            for (text, button_type) in [
                ("Buy/Upgrade", ShopButton::Buy),
                ("Sell", ShopButton::Sell),
            ] {
                parent.spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                    ..default()
                }, button_type))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        text,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            }
        });
}

pub fn cleanup_shop_ui(mut commands: Commands, query: Query<Entity, With<ShopUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &ShopButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut shop_events: EventWriter<ShopEvent>,
    mut shop_page: ResMut<ShopPage>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            *color = Color::rgb(0.35, 0.35, 0.35).into();
            match button {
                ShopButton::Buy | ShopButton::Upgrade => {
                    *shop_page = ShopPage::BuyUpgrade;
                }
                ShopButton::Sell => {
                    *shop_page = ShopPage::Sell;
                }
                ShopButton::BuyItem(index) => {
                    shop_events.send(ShopEvent::Buy(*index));
                }
                ShopButton::UpgradeItem(index) => {
                    shop_events.send(ShopEvent::Upgrade(*index));
                }
                ShopButton::SellItem(index) => {
                    shop_events.send(ShopEvent::Sell(*index));
                }
            }
        }
    }
}

pub fn update_shop_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    shop: Res<Shop>,
    player: Query<&Player>,
    shop_page: Res<ShopPage>,
    shop_ui_query: Query<Entity, With<ShopUI>>,
) {
    let player = player.single();
    let shop_ui_entity = shop_ui_query.single();

    commands.entity(shop_ui_entity).despawn_descendants();
    let font = asset_server.load("pixel_pirate.ttf");

    commands.entity(shop_ui_entity).with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "Shop",
            TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            },
        ));

        // Player's money
        parent.spawn(TextBundle::from_section(
            format!("Gold: {}", player.inventory.money),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::srgb(1.0, 0.84, 0.0),
            },
        ));

        // Shop items or player's inventory based on the current page
        match *shop_page {
            ShopPage::BuyUpgrade => {
                for (index, item) in shop.items.iter().enumerate() {
                    let owned_item = player.inventory.items.iter().find(|i| i.item_type == item.item_type);
                    let (button_text, button_type) = if let Some(owned_item) = owned_item {
                        (format!("Upgrade {} (Lvl {})", item.name, owned_item.level),
                         ShopButton::UpgradeItem(index))
                    } else {
                        (format!("Buy {}", item.name),
                         ShopButton::BuyItem(index))
                    };

                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    }, button_type))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            button_text,
                            TextStyle {
                                font: font.clone(),
                                font_size: 18.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
                }
            }
            ShopPage::Sell => {
                for (index, item) in player.inventory.items.iter().enumerate() {
                    if item.item_type == ItemType::Loot {
                        parent.spawn((ButtonBundle {
                            style: Style {
                                width: Val::Px(250.0),
                                height: Val::Px(65.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        }, ShopButton::SellItem(index)))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("Sell {} for {} gold", item.name, item.price / 2),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    }
                }
            }
        }

        // Page switch buttons
        for (text, button_type) in [
            ("Buy/Upgrade", ShopButton::Buy),
            ("Sell", ShopButton::Sell),
        ] {
            parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                ..default()
            }, button_type))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ));
            });
        }
    });
}

pub fn handle_shop_events(
    mut shop_events: EventReader<ShopEvent>,
    mut player_query: Query<&mut Player>,
    mut shop: ResMut<Shop>,
) {
    let mut player = player_query.single_mut();

    for event in shop_events.read() {
        match event {
            ShopEvent::Buy(index) => {
                if *index < shop.items.len() {
                    let item = shop.items[*index].clone();
                    if player.inventory.money >= item.price {
                        player.inventory.money -= item.price;
                        player.inventory.add_item(item);
                    }
                }
            }
            ShopEvent::Upgrade(index) => {
                if *index < shop.items.len() {
                    let shop_item = &shop.items[*index];
                    let upgrade_cost = shop_item.price / 2;
                    
                    if player.inventory.money >= upgrade_cost {
                        if player.inventory.find_and_upgrade_item(shop_item.item_type) {
                            player.inventory.money -= upgrade_cost;
                        }
                    }
                }
            }
            ShopEvent::Sell(index) => {
                if let Some(item) = player.inventory.remove_item(*index) {
                    player.inventory.money += item.price / 2;
                }
            }
        }
    }
}

pub fn generate_loot_item() -> Item {
    // temporary
    Item::new(ItemType::Loot, "Bat Wing".to_string(), 50)
}