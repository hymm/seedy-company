use bevy::prelude::*;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InventoryState>()
            .add_startup_system(spawn_items)
            .add_system(spawn_inventory.in_schedule(OnEnter(InventoryState::Disabled)))
            .add_systems(
                (color_button, selection_keyboard_input)
                    .distributive_run_if(in_state(InventoryState::Disabled)),
            );
    }
}

#[derive(States, PartialEq, Eq, Default, Debug, Hash, Clone)]
enum InventoryState {
    #[default]
    Disabled,
    Selection,
    SetPrice,
}

#[derive(Component)]
struct InventoryItem {
    handle: Handle<Image>,
}

#[derive(Component)]
struct InventoryButton;

#[derive(Resource)]
struct Inventory {
    selected_button: Entity,
    list: Vec<Entity>,
}

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

fn spawn_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 1..10 {
        commands.spawn(InventoryItem {
            handle: asset_server.load(format!("images/test-{}.png", i)),
        });
    }
}

fn spawn_inventory(mut commands: Commands, items: Query<&InventoryItem>) {
    let mut first_button = Entity::PLACEHOLDER;
    let mut list = Vec::new();
    commands
        .spawn(NodeBundle {
            style: Style {
                max_size: Size::all(Val::Px(120.)),
                align_content: AlignContent::FlexStart,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for item in items.iter() {
                let entity = builder
                    .spawn((ButtonBundle::default(), InventoryButton))
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

fn selection_keyboard_input(keys: Res<Input<KeyCode>>, mut inventory: ResMut<Inventory>) {
    if keys.just_pressed(KeyCode::Left) || keys.just_pressed(KeyCode::Up) {
        inventory.prev_item();
    }

    if keys.just_pressed(KeyCode::Right) || keys.just_pressed(KeyCode::Down) {
        inventory.next_item();
    }
}
