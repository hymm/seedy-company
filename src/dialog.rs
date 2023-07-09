use bevy::{asset::LoadState, ecs::system::Command, prelude::*};
use bevy_mod_yarn::prelude::{Dialogue, DialogueRunner, Statements, YarnAsset, YarnPlugin};

use crate::constants::{FONT, TEXT_SIZE};
pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(YarnPlugin)
            .add_event::<DialogExited>()
            .add_event::<OpenDialog>()
            .add_startup_system(spawn_dialog)
            .add_system(dialog_ready)
            .add_system(open_dialog)
            .add_system(dialogue_display)
            .add_system(dialog_input_handling);
    }
}

// marker component to get the root node of the dialog
#[derive(Component)]
struct Dialog;

// marker component to get the dialog text
#[derive(Component)]
struct DialogText;

#[derive(Component)]
struct DialogPortrait;

#[derive(Component)]
struct YarnDialog {
    pub handle: Handle<YarnAsset>,
    pub start_node: String,
}

pub struct DialogExited {
    pub node: String,
}

fn spawn_dialog(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Dialog,
            NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    flex_direction: FlexDirection::Row,
                    display: Display::None,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            // center column
            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                        },
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    // Portrait
                    builder.spawn((
                        DialogPortrait,
                        ImageBundle {
                            image: UiImage {
                                texture: asset_server.load("images/Store_Owner.png"),
                                ..default()
                            },
                            style: Style {
                                size: Size::all(Val::Px(108.)),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                    // Dialog Text
                    builder.spawn((
                        DialogText,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: asset_server.load(FONT),
                                font_size: TEXT_SIZE,
                                color: Color::rgb_u8(42, 17, 4),
                            },
                        )
                        .with_background_color(Color::rgb_u8(215, 170, 133))
                        .with_style(Style {
                            size: Size {
                                width: Val::Px(530.),
                                height: Val::Px(74.),
                            },
                            ..default()
                        }),
                    ));
                });
        });
}

fn dialog_ready(
    mut events: EventReader<AssetEvent<YarnAsset>>,
    dialogues: Res<Assets<YarnAsset>>,
    mut commands: Commands,
    dialog_text: Query<(Entity, &YarnDialog), With<DialogText>>,
    mut open_events: EventWriter<OpenDialog>,
) {
    if let Ok((e, y)) = dialog_text.get_single() {
        for event in &mut events {
            match event {
                AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                    if *handle == y.handle {
                        if let Some(dialogues) = dialogues.get(handle) {
                            commands
                                .entity(e)
                                .insert(DialogueRunner::new(dialogues.clone(), &y.start_node));
                            open_events.send(OpenDialog);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

struct OpenDialog;
fn open_dialog(
    mut commands: Commands,
    mut events: EventReader<OpenDialog>,
    mut dialog: Query<&mut Style, With<Dialog>>,
    dialog_text: Query<(Entity, &YarnDialog), With<DialogText>>,
    asset_server: Res<AssetServer>,
    dialogues: Res<Assets<YarnAsset>>,
) {
    if !events.is_empty() {
        dialog.single_mut().display = Display::Flex;

        // hacky bug fix
        if let Ok((e, yarn_dialog)) = dialog_text.get_single() {
            if asset_server.get_load_state(yarn_dialog.handle.clone_weak()) == LoadState::Loaded {
                if let Some(dialogues) = dialogues.get(&yarn_dialog.handle) {
                    commands.entity(e).insert(DialogueRunner::new(
                        dialogues.clone(),
                        &yarn_dialog.start_node,
                    ));
                    // open_events.send(OpenDialog);
                }
            }
        }
    }
    events.clear();
}

fn dialog_input_handling(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut runners: Query<&mut DialogueRunner, With<DialogText>>,
) {
    if let Ok(mut runner) = runners.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
            runner.next_entry();
        }
        if keys.just_pressed(KeyCode::Down) {
            println!("next choice");
            runner.next_choice()
        }
        if keys.just_pressed(KeyCode::Up) {
            println!("prev choice");
            runner.prev_choice()
        }
    }
}

fn dialogue_display(
    mut commands: Commands,
    mut events: EventWriter<DialogExited>,
    mut text: Query<(Entity, &mut Text, &mut DialogueRunner), With<DialogText>>,
    mut dialog: Query<&mut Style, With<Dialog>>,
) {
    if let Ok((entity, mut text, runner)) = text.get_single_mut() {
        let text = &mut text.sections[0].value;
        *text = "".to_string();
        match runner.current_statement() {
            Statements::Dialogue(Dialogue {
                who: _who, what, ..
            }) => {
                text.push_str(&format!("{}\n", what));
            }
            Statements::Choice(_) => {
                let (choices, current_choice_index) = runner.get_current_choices();
                for (index, dialogue) in choices.iter().enumerate() {
                    if index == current_choice_index {
                        text.push_str(&format!("--> {:?}: {:?}\n", dialogue.who, dialogue.what));
                    } else {
                        text.push_str(&format!("{:?}: {:?}\n", dialogue.who, dialogue.what));
                    }
                }
            }
            Statements::Exit => {
                events.send(DialogExited {
                    node: runner.current_node_name.clone(),
                });
                commands
                    .entity(entity)
                    .remove::<YarnDialog>()
                    .remove::<DialogueRunner>();
                dialog.single_mut().display = Display::None;
            }
            _ => {}
        }
    }
}

pub struct ShowDialog {
    pub handle: Handle<YarnAsset>,
    pub start_node: String,
}

impl Command for ShowDialog {
    fn write(self, world: &mut World) {
        let dialog_text_entity = world
            .query_filtered::<Entity, With<DialogText>>()
            .single(world);
        world.entity_mut(dialog_text_entity).insert(YarnDialog {
            handle: self.handle,
            start_node: self.start_node,
        });
        world.resource_mut::<Events<OpenDialog>>().send(OpenDialog);
    }
}
