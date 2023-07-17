use bevy::prelude::*;

use crate::{
    dialog::{DialogExited, ShowDialog},
    game_state::{GameState, StoreSetupState},
};

pub struct RunningPlugin;
impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StoreSetup), transition_store_setup)
            .add_systems(OnEnter(StoreSetupState::OpeningDialog), introduction)
            .add_systems(
                Update,
                welcome_done.run_if(in_state(StoreSetupState::OpeningDialog)),
            );
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
