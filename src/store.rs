use std::collections::VecDeque;

use crate::{
    constants::{FONT, TEXT_SIZE},
    dialog::{DialogExited, ShowDialog},
    game_state::{GameState, StoreSetupState},
    inventory::ActiveItem,
};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct StorePlugin;
impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StoreSetupState::PedestalSelect),
            spawn_pedestals.run_if(|q: Query<&ItemDisplay>| q.is_empty()),
        )
        .add_systems(
            OnEnter(StoreSetupState::PedestalSelect),
            FinishButton::spawn,
        )
        .add_systems(
            Update,
            FinishButton::interaction_handler.run_if(in_state(StoreSetupState::PedestalSelect)),
        )
        .add_systems(
            OnExit(StoreSetupState::PedestalSelect),
            FinishButton::despawn,
        )
        .add_systems(
            Update,
            handle_pedestal_click.run_if(in_state(StoreSetupState::PedestalSelect)),
        );

        app.add_systems(OnEnter(StoreSetupState::FarmerBuy), show_farmer_dialog)
            .add_systems(
                Update,
                farmer_buy_done.run_if(in_state(StoreSetupState::FarmerBuy)),
            );

        app.add_systems(OnEnter(GameState::StoreSetup), Store::spawn_background)
            .add_systems(OnExit(GameState::StoreSetup), Store::despawn_store);
    }
}

#[derive(Component)]
pub struct Store;

impl Store {
    fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Store,
            SpriteBundle {
                texture: asset_server.load("images/Store_Spring.png"),
                ..default()
            },
        ));
    }

    fn despawn_store(mut commands: Commands, store: Query<Entity, With<Store>>) {
        for e in &store {
            commands.entity(e).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub struct ItemDisplay;
impl ItemDisplay {
    const SIZE: f32 = 24.;
    fn point_inside(point: Vec2, transform: &Transform) -> bool {
        let center = transform.translation.truncate();
        point.x > center.x - Self::SIZE / 2.
            && point.x < center.x + Self::SIZE / 2.
            && point.y > center.y - Self::SIZE / 2.
            && point.y < center.y + Self::SIZE / 2.
    }
}

#[derive(Resource)]
pub struct SelectedPedestal(pub Entity);

fn spawn_pedestals(mut commands: Commands) {
    for i in 0..3 {
        commands.spawn((
            ItemDisplay,
            Store,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(ItemDisplay::SIZE, ItemDisplay::SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(i as f32 * 48.0, 0., 1.)),
                ..default()
            },
        ));
    }
}

#[derive(Component)]
struct FinishButton;

#[derive(Component)]
struct FinishButtonMarker;

impl FinishButton {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                FinishButtonMarker,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|builder| {
                builder
                    .spawn((
                        FinishButton,
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(530.),
                                height: Val::Px(74.),
                                ..default()
                            },
                            background_color: Color::rgb_u8(215, 170, 133).into(),
                            ..default()
                        },
                    ))
                    .with_children(|child| {
                        child.spawn(TextBundle::from_section(
                    "Click on Barrels to select items to sell\nClick here when done to continue.",
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: TEXT_SIZE,
                        color: Color::rgb_u8(42, 17, 4),
                    },
                ));
                    });
            });
    }

    fn despawn(mut commands: Commands, finish_button: Query<Entity, With<FinishButtonMarker>>) {
        for e in &finish_button {
            commands.entity(e).despawn_recursive();
        }
    }

    fn interaction_handler(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<FinishButton>),
        >,
        mut state: ResMut<NextState<StoreSetupState>>,
    ) {
        for (interaction, mut _color) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    state.set(StoreSetupState::FarmerBuy);
                }
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        }
    }
}

fn handle_pedestal_click(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    displays: Query<(Entity, &Transform), With<ItemDisplay>>,
    mut state: ResMut<NextState<StoreSetupState>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();

        let window = window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            for (e, display_transform) in &displays {
                if ItemDisplay::point_inside(world_position, display_transform) {
                    commands.insert_resource(SelectedPedestal(e));
                    state.set(StoreSetupState::Inventory);
                }
            }
        }
    }
}

fn show_farmer_dialog(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.add(ShowDialog {
        handle: asset_server.load("dialogs/basic.yarn"),
        start_node: "FarmerBuy".into(),
    });
}

#[derive(Resource)]
pub struct ActiveItems {
    pub items: VecDeque<ActiveItem>,
}

fn farmer_buy_done(
    mut commands: Commands,
    mut events: EventReader<DialogExited>,
    mut state: ResMut<NextState<GameState>>,
    active_items: Query<&ActiveItem>,
) {
    for event in &mut events {
        if &event.node == "FarmerBuy" {
            let items = active_items.iter().copied().collect();
            commands.insert_resource(ActiveItems { items });

            state.set(GameState::FarmingBattle);
        }
    }
}
