use crate::{
    constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    dialog::{DialogExited, ShowDialog},
    game_state::{GameState, StoreSetupState},
};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct StorePlugin;
impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_pedestals
                .in_schedule(OnEnter(StoreSetupState::PedestalSelect))
                .run_if(|q: Query<&ItemDisplay>| q.is_empty()),
        )
        .add_system(FinishButton::spawn.in_schedule(OnEnter(StoreSetupState::PedestalSelect)))
        .add_system(
            FinishButton::interaction_handler.run_if(in_state(StoreSetupState::PedestalSelect)),
        )
        .add_system(FinishButton::despawn.in_schedule(OnExit(StoreSetupState::PedestalSelect)))
        .add_system(handle_pedestal_click.run_if(in_state(StoreSetupState::PedestalSelect)));

        app.add_system(show_farmer_dialog.in_schedule(OnEnter(StoreSetupState::FarmerBuy)))
            .add_system(farmer_buy_done.run_if(in_state(StoreSetupState::FarmerBuy)));

        app.add_system(despawn_store.in_schedule(OnExit(GameState::StoreSetup)));
    }
}

#[derive(Component)]
pub struct Store;

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
                transform: Transform::from_translation(Vec3::new(i as f32 * 48.0, 0., 0.)),
                ..default()
            },
        ));
    }
}

#[derive(Component)]
struct FinishButton;

impl FinishButton {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                FinishButton,
                ButtonBundle {
                    style: Style {
                        // size: Size::all(Val::Percent(100.)),
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },
            ))
            .with_children(|child| {
                child.spawn(TextBundle::from_section(
                    "Finished",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                ));
            });
    }

    fn despawn(mut commands: Commands, finish_button: Query<Entity, With<FinishButton>>) {
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
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    state.set(StoreSetupState::FarmerBuy);
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

fn farmer_buy_done(mut events: EventReader<DialogExited>, mut state: ResMut<NextState<GameState>>) {
    for event in &mut events {
        if &event.node == "FarmerBuy" {
            state.set(GameState::FarmingBattle);
        }
    }
}

fn despawn_store(mut commands: Commands, store: Query<Entity, With<Store>>) {
    for e in &store {
        commands.entity(e).despawn_recursive();
    }
}
