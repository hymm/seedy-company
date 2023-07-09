// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod constants;
mod dialog;
mod farm;
mod game_state;
mod inventory;
mod running;
mod start_menu;
mod store;

use bevy::prelude::*;
use dialog::DialogPlugin;
use farm::FarmPlugin;
use game_state::GameStatePlugin;
use inventory::InventoryPlugin;
use running::RunningPlugin;
use start_menu::StartMenuPlugin;
use store::StorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DialogPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(StartMenuPlugin)
        .add_plugin(RunningPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(StorePlugin)
        .add_plugin(FarmPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
