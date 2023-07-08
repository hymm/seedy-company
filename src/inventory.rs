use bevy::prelude::*;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InventoryState>()
            .add_startup_system(spawn_items);

        // Item Selection systems
        app.add_system(spawn_inventory.in_schedule(OnEnter(InventoryState::Selection)))
            .add_systems(
                (color_button, selection_keyboard_input)
                    .distributive_run_if(in_state(InventoryState::Selection)),
            )
            .add_system(despawn_inventory_ui.in_schedule(OnExit(InventoryState::Selection)));

        // Price Setter Systems
        app.add_system(spawn_price_setter.in_schedule(OnEnter(InventoryState::SetPrice)));
    }
}

#[derive(States, PartialEq, Eq, Default, Debug, Hash, Clone)]
pub enum InventoryState {
    #[default]
    Disabled,
    Selection,
    SetPrice,
}

#[derive(Component)]
struct InventoryItem {
    handle: Handle<Image>,
    name: String,
}

#[derive(Component)]
struct InventoryButton {
    /// Points to an entity with a `InventoryItem`
    item: Entity,
}

#[derive(Resource)]
struct Inventory {
    selected_button: Entity,
    // list of button entites
    list: Vec<Entity>,
}

#[derive(Component)]
struct InventoryUi;

impl Inventory {
    fn next_item(&mut self) {
        let selected_index = self
            .list
            .iter()
            .position(|e| *e == self.selected_button)
            .unwrap();
        self.selected_button = if selected_index + 1 == self.list.len() {
            self.list[0]
        } else {
            self.list[selected_index + 1]
        }
    }

    fn prev_item(&mut self) {
        let selected_index = self
            .list
            .iter()
            .position(|e| *e == self.selected_button)
            .unwrap();
        self.selected_button = if selected_index == 0 {
            self.list[self.list.len() - 1]
        } else {
            self.list[selected_index - 1]
        }
    }
}

const INVENTORY_ITEMS: [(&str, &str); 9] = [
    ("Item 1", "images/test-1.png"),
    ("Item 2", "images/test-2.png"),
    ("Item 3", "images/test-3.png"),
    ("Item 4", "images/test-4.png"),
    ("Item 5", "images/test-5.png"),
    ("Item 6", "images/test-6.png"),
    ("Item 7", "images/test-7.png"),
    ("Item 8", "images/test-8.png"),
    ("Item 9", "images/test-9.png"),
];

fn spawn_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    for (name, path) in INVENTORY_ITEMS {
        commands.spawn(InventoryItem {
            handle: asset_server.load(path),
            name: name.into(),
        });
    }
}

fn spawn_inventory(mut commands: Commands, items: Query<(Entity, &InventoryItem)>) {
    let mut first_button = Entity::PLACEHOLDER;
    let mut list = Vec::new();
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
            for (e, item) in items.iter() {
                let entity = builder
                    .spawn((ButtonBundle::default(), InventoryButton { item: e }))
                    .with_children(|builder| {
                        builder.spawn(ImageBundle {
                            style: Style {
                                size: Size::all(Val::Px(24.)),
                                ..default()
                            },
                            image: UiImage {
                                texture: item.handle.clone(),
                                ..default()
                            },
                            ..default()
                        });
                    })
                    .id();
                list.push(entity);
                if first_button == Entity::PLACEHOLDER {
                    first_button = entity;
                }
            }
        });
    commands.insert_resource(Inventory {
        selected_button: first_button,
        list,
    });
}

fn color_button(
    mut buttons: Query<(Entity, &mut BackgroundColor), With<InventoryButton>>,
    inventory: ResMut<Inventory>,
) {
    for (e, mut color) in &mut buttons {
        *color = if inventory.selected_button == e {
            Color::BISQUE.into()
        } else {
            Color::GRAY.into()
        };
    }
}

fn selection_keyboard_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut inventory: ResMut<Inventory>,
    mut state: ResMut<NextState<InventoryState>>,
    item_buttons: Query<&InventoryButton>,
) {
    if keys.just_pressed(KeyCode::Left) || keys.just_pressed(KeyCode::Up) {
        inventory.prev_item();
    }

    if keys.just_pressed(KeyCode::Right) || keys.just_pressed(KeyCode::Down) {
        inventory.next_item();
    }

    if keys.just_pressed(KeyCode::Space) {
        let item_entity = item_buttons.get(inventory.selected_button).unwrap().item;
        commands.insert_resource(SetPriceFor(item_entity));
        state.set(InventoryState::SetPrice);
    }
}

fn despawn_inventory_ui(ui: Query<Entity, With<InventoryUi>>, mut commands: Commands) {
    for e in &ui {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
struct PriceSetterUi;

#[derive(Resource)]
struct SetPriceFor(Entity);

fn spawn_price_setter(
    mut commands: Commands,
    items: Query<&InventoryItem>,
    set_price_for: Res<SetPriceFor>,
    asset_server: Res<AssetServer>,
) {
    let item = items.get(set_price_for.0).unwrap();

    let default_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 50.,
        color: Color::WHITE,
    };

    commands
        .spawn((
            PriceSetterUi,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            // icon
            builder.spawn(ImageBundle {
                image: UiImage {
                    texture: item.handle.clone(),
                    ..default()
                },
                ..default()
            });
            // item name
            builder.spawn(TextBundle::from_section(
                item.name.clone(),
                default_text_style.clone(),
            ));
            // price
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|price_builder| {
                    // minus button
                    price_builder
                        .spawn(ButtonBundle {
                            background_color: Color::GRAY.into(),
                            ..default()
                        })
                        .with_children(|minus_builder| {
                            minus_builder
                                .spawn(TextBundle::from_section("-", default_text_style.clone()));
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
                            builder.spawn(TextBundle::from_section(
                                format!("{}", 0.0),
                                default_text_style.clone(),
                            ));
                        });

                    // plus button
                    price_builder
                        .spawn(ButtonBundle {
                            background_color: Color::GRAY.into(),
                            ..default()
                        })
                        .with_children(|minus_builder| {
                            minus_builder
                                .spawn(TextBundle::from_section("+", default_text_style.clone()));
                        });
                });
        });
}
