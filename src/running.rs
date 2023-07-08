use bevy::prelude::*;

use crate::{
    dialog::{DialogExited, ShowDialog},
    game_state::{GameState, StoreSetupState},
};

pub struct RunningPlugin;
impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(transition_store_setup.in_schedule(OnEnter(GameState::StoreSetup)))
            .add_system(introduction.in_schedule(OnEnter(StoreSetupState::OpeningDialog)))
            .add_system(welcome_done.run_if(in_state(StoreSetupState::OpeningDialog)));
    }
}

fn transition_store_setup(mut state: ResMut<NextState<StoreSetupState>>) {
    state.set(StoreSetupState::OpeningDialog);
}

fn introduction(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.add(ShowDialog {
        handle: asset_server.load("dialogs/basic.yarn"),
        start_node: "Welcome".into(),
    });
}

fn welcome_done(
    mut events: EventReader<DialogExited>,
    mut state: ResMut<NextState<StoreSetupState>>,
) {
    for event in &mut events {
        if &event.node == "Welcome" {
            state.set(StoreSetupState::PedestalSelect);
        }
    }
}
