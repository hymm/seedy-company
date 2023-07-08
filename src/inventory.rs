use crate::{
    constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    game_state::StoreSetupState,
    store::{ItemDisplay, SelectedPedestal},
};
use bevy::prelude::*;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_items);

        // Item Selection systems
        app.add_system(spawn_inventory.in_schedule(OnEnter(StoreSetupState::Inventory)))
            .add_systems(
                (selection_mouse_handler, || {})
                    .distributive_run_if(in_state(StoreSetupState::Inventory)),
            )
            .add_system(despawn_inventory_ui.in_schedule(OnExit(StoreSetupState::Inventory)));

        // Price Setter Systems
        app.add_system(spawn_price_setter.in_schedule(OnEnter(StoreSetupState::PriceSelect)))
            .add_systems(
                (
                    CloseButton::interaction_handler,
                    PriceDisplay::update_text,
                    PriceDisplay::handle_minus_interaction,
                    PriceDisplay::handle_plus_interaction,
                    QuantityDisplay::update_text,
                    QuantityDisplay::handle_minus_interaction,
                    QuantityDisplay::handle_plus_interaction,
                    DoneButton::handle_interaction,
                )
                    .distributive_run_if(in_state(StoreSetupState::PriceSelect)),
            )
            .add_system(despawn_price_setter.in_schedule(OnExit(StoreSetupState::PriceSelect)));
    }
}

#[derive(Component)]
struct InventoryButton {
    /// Points to an entity with a `InventoryItem`
    item: Entity,
}

#[derive(Resource)]
struct Inventory;

#[derive(Component)]
struct InventoryUi;

#[derive(Component, Clone)]
pub struct SellableItem {
    name: &'static str,
    icon_path: &'static str,
    description: &'static str,
    // price it costs shopkeeper
    store_price: i32,
    // price it can be bought back for
    buy_back_price: i32,
}

const SELLABLE_ITEMS: [SellableItem; 9] = [
    SellableItem {
        name: "Hoe",
        icon_path: "images/test-1.png",
        description: "Used to till the ground.",
        store_price: 100,
        buy_back_price: 0,
    },
    SellableItem {
        name: "Watering Can",
        icon_path: "images/test-2.png",
        description: "Used for watering plants",
        store_price: 50,
        buy_back_price: 0,
    },
    SellableItem {
        name: "Scythe",
        icon_path: "images/test-3.png",
        description: "Used for harvesting plants",
        store_price: 75,
        buy_back_price: 0,
    },
    SellableItem {
        name: "Parsnip Seeds",
        icon_path: "images/test-4.png",
        description: "Grows in 3 days, Sells for 400g",
        store_price: 20,
        buy_back_price: 40,
    },
    SellableItem {
        name: "Blueberry Seeds",
        icon_path: "images/test-5.png",
        description: "Item 5",
        store_price: 18,
        buy_back_price: 50,
    },
    SellableItem {
        name: "Item 6",
        icon_path: "images/test-6.png",
        description: "Item 6",
        store_price: 230,
        buy_back_price: 0,
    },
    SellableItem {
        name: "Item 7",
        icon_path: "images/test-7.png",
        description: "Item 7",
        store_price: 230,
        buy_back_price: 0,
    },
    SellableItem {
        name: "Item 8",
        icon_path: "images/test-8.png",
        description: "Item 8",
        store_price: 230,
        buy_back_price: 0,
    },
    SellableItem {
        name: "Item 9",
        icon_path: "images/test-9.png",
        description: "Item 9",
        store_price: 230,
        buy_back_price: 0,
    },
];

fn spawn_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    for item in SELLABLE_ITEMS {
        let handle: Handle<Image> = asset_server.load(item.icon_path);
        commands.spawn((item, handle));
    }
}

fn spawn_inventory(
    mut commands: Commands,
    items: Query<(Entity, &Handle<Image>), With<SellableItem>>,
) {
    commands
        .spawn((
            InventoryUi,
            NodeBundle {
                style: Style {
                    max_size: Size::all(Val::Px(120.)),
                    align_content: AlignContent::FlexStart,
                    flex_wrap: FlexWrap::Wrap,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            for (e, handle) in items.iter() {
                builder
                    .spawn((ButtonBundle::default(), InventoryButton { item: e }))
                    .with_children(|builder| {
                        builder.spawn(ImageBundle {
                            style: Style {
                                size: Size::all(Val::Px(24.)),
                                ..default()
                            },
                            image: UiImage {
                                texture: handle.clone(),
                                ..default()
                            },
                            ..default()
                        });
                    });
            }
        });
}

fn selection_mouse_handler(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<InventoryButton>),
    >,
    item_buttons: Query<&InventoryButton>,
    mut state: ResMut<NextState<StoreSetupState>>,
) {
    for (e, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let item_entity = item_buttons.get(e).unwrap().item;
                commands.insert_resource(SetPriceFor(item_entity));
                state.set(StoreSetupState::PriceSelect);
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn despawn_inventory_ui(ui: Query<Entity, With<InventoryUi>>, mut commands: Commands) {
    for e in &ui {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
struct PriceSetterUi {
    max_quantity: i32,
    min_quantity: i32,
    quantity: i32,
    sell_at: i32,
    store_price: i32,
}

#[derive(Resource)]
struct SetPriceFor(Entity);

fn spawn_price_setter(
    mut commands: Commands,
    items: Query<(&SellableItem, &Handle<Image>)>,
    set_price_for: Res<SetPriceFor>,
    asset_server: Res<AssetServer>,
) {
    let (item, item_image_handle) = items.get(set_price_for.0).unwrap();

    let default_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 50.,
        color: Color::WHITE,
    };

    commands
        .spawn((
            PriceSetterUi {
                max_quantity: 5,
                min_quantity: 1,
                quantity: 1,
                sell_at: item.store_price,
                store_price: item.store_price,
            },
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            // Title Bar
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|title_bar| {
                    // Title
                    title_bar.spawn(TextBundle::from_section(
                        "Set Price",
                        default_text_style.clone(),
                    ));

                    // close button
                    CloseButton::spawn(title_bar, &asset_server);
                });
            // icon
            builder.spawn(ImageBundle {
                image: UiImage {
                    texture: item_image_handle.clone(),
                    ..default()
                },
                style: Style {
                    max_size: Size::all(Val::Px(24.)),
                    ..default()
                },
                ..default()
            });
            // item name
            builder.spawn(TextBundle::from_section(
                item.name,
                default_text_style.clone(),
            ));
            // description
            builder.spawn(TextBundle::from_section(
                item.description,
                default_text_style.clone(),
            ));
            // Cost per Item/Use
            CostText::spawn(builder, &asset_server, item.store_price);
            // quantity
            QuantityDisplay::spawn(builder, &asset_server);
            // price
            PriceDisplay::spawn(builder, &asset_server);

            builder
                .spawn((
                    DoneButton,
                    ButtonBundle {
                        background_color: Color::GRAY.into(),
                        style: Style {
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|minus_builder| {
                    minus_builder
                        .spawn(TextBundle::from_section("Done", default_text_style.clone()));
                });
        });
}

fn despawn_price_setter(ui: Query<Entity, With<PriceSetterUi>>, mut commands: Commands) {
    for e in &ui {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
struct CloseButton;
impl CloseButton {
    fn spawn(builder: &mut ChildBuilder<'_, '_, '_>, asset_server: &AssetServer) {
        builder
            .spawn((
                CloseButton,
                ButtonBundle {
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(20.),
                            ..default()
                        },
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },
            ))
            .with_children(|minus_builder| {
                minus_builder.spawn(TextBundle::from_section(
                    "x",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                ));
            });
    }

    fn interaction_handler(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<CloseButton>),
        >,
        mut state: ResMut<NextState<StoreSetupState>>,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    state.set(StoreSetupState::Inventory);
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}

struct CostText;
impl CostText {
    fn spawn(builder: &mut ChildBuilder<'_, '_, '_>, asset_server: &AssetServer, cost: i32) {
        let text_style = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 50.,
            color: Color::WHITE,
        };
        builder
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|child_builder| {
                child_builder
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::all(Val::Percent(100.)),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            flex_grow: 1.,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section(
                            "Cost/Item(Use)",
                            text_style.clone(),
                        ));
                    });

                // cost
                child_builder
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::all(Val::Percent(100.)),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            flex_grow: 1.,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section(
                            format!("{}g", cost),
                            text_style.clone(),
                        ));
                    });
            });
    }
}

#[derive(Component)]
struct PriceDisplay;

#[derive(Component)]
struct PriceDisplayMinus;

#[derive(Component)]
struct PriceDisplayPlus;

impl PriceDisplay {
    const INCREMENT: i32 = 50;
    fn spawn(builder: &mut ChildBuilder<'_, '_, '_>, asset_server: &AssetServer) {
        let text_style = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 50.,
            color: Color::WHITE,
        };
        builder
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|price_builder| {
                // line header
                price_builder
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::all(Val::Percent(100.)),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            flex_grow: 1.,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section("Sell At", text_style.clone()));
                    });

                // plus button
                price_builder
                    .spawn((
                        PriceDisplayPlus,
                        ButtonBundle {
                            background_color: Color::GRAY.into(),
                            ..default()
                        },
                    ))
                    .with_children(|minus_builder| {
                        minus_builder.spawn(TextBundle::from_section("+", text_style.clone()));
                    });

                // price text
                price_builder
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::all(Val::Percent(100.)),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            flex_grow: 1.,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            PriceDisplay,
                            TextBundle::from_section(format!("{}", 0.0), text_style.clone()),
                        ));
                    });

                // minus button
                price_builder
                    .spawn((
                        PriceDisplayMinus,
                        ButtonBundle {
                            background_color: Color::GRAY.into(),
                            ..default()
                        },
                    ))
                    .with_children(|minus_builder| {
                        minus_builder.spawn(TextBundle::from_section("-", text_style.clone()));
                    });
            });
    }

    fn update_text(
        price: Query<&PriceSetterUi, Changed<PriceSetterUi>>,
        mut text: Query<&mut Text, With<PriceDisplay>>,
    ) {
        if let Ok(price) = price.get_single() {
            let text = &mut text.single_mut().sections[0].value;
            *text = format!("{}g", price.sell_at);
        }
    }

    fn handle_minus_interaction(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<PriceDisplayMinus>),
        >,
        mut price: Query<&mut PriceSetterUi>,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    let mut price = price.single_mut();
                    price.sell_at -= Self::INCREMENT;
                    if price.sell_at < 0 {
                        price.sell_at = 0;
                    }
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }

    fn handle_plus_interaction(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<PriceDisplayPlus>),
        >,
        mut price: Query<&mut PriceSetterUi>,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    let mut price = price.single_mut();
                    price.sell_at += Self::INCREMENT;
                    if price.sell_at > 10000 {
                        price.sell_at = 10000;
                    }
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}

#[derive(Component)]
struct QuantityDisplay;
#[derive(Component)]
struct QuantityDisplayMinus;
#[derive(Component)]
struct QuantityDisplayPlus;
impl QuantityDisplay {
    const INCREMENT: i32 = 1;
    fn spawn(builder: &mut ChildBuilder<'_, '_, '_>, asset_server: &AssetServer) {
        let text_style = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 50.,
            color: Color::WHITE,
        };
        builder
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|price_builder| {
                // line header
                price_builder
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::all(Val::Percent(100.)),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            flex_grow: 1.,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section("Quantity", text_style.clone()));
                    });

                // plus button
                price_builder
                    .spawn((
                        QuantityDisplayPlus,
                        ButtonBundle {
                            background_color: Color::GRAY.into(),
                            ..default()
                        },
                    ))
                    .with_children(|minus_builder| {
                        minus_builder.spawn(TextBundle::from_section("+", text_style.clone()));
                    });

                // price text
                price_builder
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::all(Val::Percent(100.)),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexEnd,
                            flex_grow: 1.,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            QuantityDisplay,
                            TextBundle::from_section(format!("{}", 0.0), text_style.clone()),
                        ));
                    });

                // minus button
                price_builder
                    .spawn((
                        QuantityDisplayMinus,
                        ButtonBundle {
                            background_color: Color::GRAY.into(),
                            ..default()
                        },
                    ))
                    .with_children(|minus_builder| {
                        minus_builder.spawn(TextBundle::from_section("-", text_style.clone()));
                    });
            });
    }

    fn update_text(
        price: Query<&PriceSetterUi, Changed<PriceSetterUi>>,
        mut text: Query<&mut Text, With<QuantityDisplay>>,
    ) {
        if let Ok(price) = price.get_single() {
            let text = &mut text.single_mut().sections[0].value;
            *text = format!("{}", price.quantity);
        }
    }

    fn handle_minus_interaction(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<QuantityDisplayMinus>),
        >,
        mut price: Query<&mut PriceSetterUi>,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    let mut price = price.single_mut();
                    price.quantity = (price.quantity - Self::INCREMENT).max(price.min_quantity);
                    price.sell_at = price.sell_at.max(price.quantity * price.store_price);
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }

    fn handle_plus_interaction(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<QuantityDisplayPlus>),
        >,
        mut price: Query<&mut PriceSetterUi>,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    let mut price = price.single_mut();
                    price.quantity = (price.quantity + Self::INCREMENT).min(price.max_quantity);
                    price.sell_at = price.sell_at.max(price.quantity * price.store_price);

                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}

#[derive(Component)]
struct DoneButton;
impl DoneButton {
    fn handle_interaction(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<DoneButton>),
        >,
        mut state: ResMut<NextState<StoreSetupState>>,
        set_price_for: Res<SetPriceFor>,
        sellables: Query<&SellableItem>,
        selected_pedestal: Res<SelectedPedestal>,
        mut pedestals: Query<(Entity, &mut Sprite, &mut Handle<Image>), With<ItemDisplay>>,
        asset_server: Res<AssetServer>,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    let item = sellables.get(set_price_for.0).unwrap();
                    let (pedestal_entity, mut pedestal_sprite, mut pedestal_texture) =
                        pedestals.get_mut(selected_pedestal.0).unwrap();
                    *pedestal_texture = asset_server.load(item.icon_path);
                    pedestal_sprite.color = Color::default();
                    state.set(StoreSetupState::PedestalSelect);
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}
