// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod dialog;
mod game_state;
mod running;
mod start_menu;
mod inventory;

use bevy::prelude::*;
use dialog::DialogPlugin;
use game_state::GameStatePlugin;
use running::RunningPlugin;
use start_menu::StartMenuPlugin;
use inventory::InventoryPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DialogPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(StartMenuPlugin)
        .add_plugin(RunningPlugin)
        .add_plugin(InventoryPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}
