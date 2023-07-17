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

use bevy::{prelude::*, window::WindowResolution};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use dialog::DialogPlugin;
use farm::FarmPlugin;
use game_state::GameStatePlugin;
use inventory::InventoryPlugin;
use running::RunningPlugin;
use start_menu::StartMenuPlugin;
use store::StorePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(640., 360.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            PixelCameraPlugin,
            DialogPlugin,
            GameStatePlugin,
            StartMenuPlugin,
            RunningPlugin,
            InventoryPlugin,
            StorePlugin,
            FarmPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_zoom(2));
}
