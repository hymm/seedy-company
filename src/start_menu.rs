use crate::constants::{FONT, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::game_state::GameState;
use bevy::prelude::*;

pub struct StartMenuPlugin;
impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu.in_schedule(OnEnter(GameState::Start)))
            .add_systems(
                (input_start, button_system).distributive_run_if(in_state(GameState::Start)),
            )
            .add_system(despawn_menu.in_schedule(OnExit(GameState::Start)));
    }
}

#[derive(Component)]
pub struct MenuMarker;

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            MenuMarker,
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    MenuMarker,
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::End,
                            margin: UiRect::bottom(Val::Px(24.0)),
                            ..default()
                        },
                        background_color: Color::rgba_u8(255, 255, 255, 0).into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuMarker,
                        TextBundle::from_section(
                            "Click to Start",
                            TextStyle {
                                font: asset_server.load(FONT),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                    ));
                });
        });

    commands.spawn((
        MenuMarker,
        SpriteBundle {
            texture: asset_server.load("images/Start_Screen_Logo.png"),
            ..default()
        },
    ));
}

fn despawn_menu(mut commands: Commands, q: Query<Entity, With<MenuMarker>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            state.set(GameState::StoreSetup);
        }
    }
}

fn input_start(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    button_inputs: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Return) {
        state.set(GameState::StoreSetup);
    }

    for gamepad in gamepads.iter() {
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::Start))
            || button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            state.set(GameState::StoreSetup);
        }
    }
}
