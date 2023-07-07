use bevy::prelude::*;
use bevy_mod_yarn::prelude::{Dialogue, DialogueRunner, Statements, YarnAsset, YarnPlugin};
pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(YarnPlugin)
            .add_startup_system(spawn_dialog)
            .add_system(dialog_ready)
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
struct YarnDialog(pub Handle<YarnAsset>);

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
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            // left margin
            builder.spawn(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(5.),
                        height: Val::Percent(100.),
                    },
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            });

            // center column
            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                        },
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(NodeBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(100.),
                                height: Val::Percent(70.),
                            },
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    });
                    // Dialog Text
                    builder.spawn((
                        DialogText,
                        YarnDialog(asset_server.load("dialogs/basic.yarn")),
                        TextBundle::from_section(
                            "blah",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 50.,
                                color: Color::WHITE,
                            },
                        )
                        .with_background_color(Color::BLUE)
                        .with_style(Style {
                            flex_grow: 1.,
                            ..default()
                        }),
                    ));
                    builder.spawn(NodeBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(100.),
                                height: Val::Percent(5.),
                            },
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    });
                });

            // right margin
            builder.spawn(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(5.),
                        height: Val::Percent(100.),
                    },
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            });
        });
}

fn dialog_ready(
    mut events: EventReader<AssetEvent<YarnAsset>>,
    dialogues: Res<Assets<YarnAsset>>,
    mut commands: Commands,
    dialog: Query<(Entity, &YarnDialog), With<DialogText>>,
) {
    if let Ok((e, y)) = dialog.get_single() {
        for event in &mut events {
            if let AssetEvent::Created { handle } = event {
                if *handle == y.0 {
                    if let Some(dialogues) = dialogues.get(handle) {
                        commands
                            .entity(e)
                            .insert(DialogueRunner::new(dialogues.clone(), "Start"));
                    }
                }
            };
        }
    }
}

fn dialog_input_handling(
    keys: Res<Input<KeyCode>>,
    mut runners: Query<&mut DialogueRunner, With<DialogText>>,
) {
    if let Ok(mut runner) = runners.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) {
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
    mut text: Query<(&mut Text, &mut DialogueRunner), With<DialogText>>,
    dialog: Query<Entity, With<Dialog>>,
) {
    if let Ok((mut text, runner)) = text.get_single_mut() {
        let text = &mut text.sections[0].value;
        *text = "".to_string();
        match runner.current_statement() {
            Statements::Dialogue(Dialogue { who, what, .. }) => {
                text.push_str(&format!("{}: {}\n", who, what));
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
                commands.entity(dialog.single()).despawn_recursive();
            }
            _ => {}
        }
    }
}
