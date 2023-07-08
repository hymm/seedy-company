use bevy::prelude::*;

use crate::{
    dialog::{DialogExited, ShowDialog},
    game_state::GameState,
    inventory::InventoryState,
};

pub struct RunningPlugin;
impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(introduction.in_schedule(OnEnter(GameState::Running)))
            .add_system(welcome_done.run_if(in_state(InventoryState::Disabled)));
    }
}

fn introduction(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.add(ShowDialog {
        handle: asset_server.load("dialogs/basic.yarn"),
        start_node: "Welcome".into(),
    });
}

fn welcome_done(
    mut events: EventReader<DialogExited>,
    mut inventory_state: ResMut<NextState<InventoryState>>,
) {
    for event in &mut events {
        if &event.node == "Welcome" {
            inventory_state.set(InventoryState::Selection);
        }
    }
}
