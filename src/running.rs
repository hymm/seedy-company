use bevy::prelude::*;

use crate::{dialog::ShowDialog, game_state::GameState};

pub struct RunningPlugin;
impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(introduction.in_schedule(OnEnter(GameState::Running)));
    }
}

fn introduction(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.add(ShowDialog {
        handle: asset_server.load("dialogs/basic.yarn"),
        start_node: "Welcome".into(),
    });
}
