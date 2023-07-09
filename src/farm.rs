use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    dialog::{DialogExited, ShowDialog},
    game_state::{FarmingBattleState, GameState},
    store::ActiveItems,
};

pub struct FarmPlugin;
impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(FarmState::spawn_farm.in_schedule(OnEnter(GameState::FarmingBattle)))
            .add_system(
                apply_active_item
                    .run_if(in_state(FarmingBattleState::ApplyItems))
                    .run_if(on_timer(Duration::from_secs_f32(0.2))),
            )
            .add_system(active_items_done.run_if(in_state(FarmingBattleState::ApplyItems)))
            .add_system(enter_summary.in_schedule(OnEnter(FarmingBattleState::ShowSummary)))
            .add_system(after_summary.run_if(in_state(FarmingBattleState::ShowSummary)))
            .add_system(FarmState::despawn_farm.in_schedule(OnExit(GameState::FarmingBattle)))
            .add_system(back_to_store.in_schedule(OnEnter(FarmingBattleState::DelayTransition)));
    }
}

#[derive(Component, Clone, Copy)]
enum FarmTile {
    Dirt,
    Tilled,
    Seeded,
    SproutedDry,
    SproutedWet,
    FullGrown,
    Failed,
}
impl FarmTile {
    fn get_asset_path(&self) -> &str {
        match self {
            FarmTile::Dirt => "images/Dirt_Tile.png",
            FarmTile::Tilled => "images/Tilled_Tile.png",
            FarmTile::Seeded => "images/Seed_Tile.png",
            FarmTile::SproutedDry => "images/Dry_Sprout_Tile.png",
            FarmTile::SproutedWet => "images/Wet_Sprout_Tile.png",
            FarmTile::FullGrown => "images/Blueberry_Tile.png",
            FarmTile::Failed => "images/Fail_Sprout_Tile.png",
        }
    }
}

#[derive(Resource, Clone)]
struct FarmState {
    tiles: [FarmTile; 25],
}
impl FarmState {
    fn spawn_farm(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        farm_state: Option<ResMut<FarmState>>,
    ) {
        let farm_state = if let Some(farm_state) = farm_state {
            farm_state.clone()
        } else {
            FarmState {
                tiles: [FarmTile::Dirt; 25],
            }
        };
        const TILE_SIZE: f32 = 24.;
        let start_pos = -TILE_SIZE * 2.;
        for i in 0..5 {
            for j in 0..5 {
                let tile = farm_state.tiles[j + i * 5];
                commands.spawn((
                    tile,
                    SpriteBundle {
                        texture: asset_server.load(tile.get_asset_path()),
                        transform: Transform::from_xyz(
                            start_pos + TILE_SIZE * i as f32,
                            start_pos + TILE_SIZE * j as f32,
                            0.0,
                        ),
                        ..default()
                    },
                ));
            }
        }

        commands.insert_resource(farm_state);
    }

    fn despawn_farm(mut commands: Commands, q: Query<Entity, With<FarmTile>>) {
        for e in &q {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn apply_active_item(mut active_items: ResMut<ActiveItems>, mut farm_state: ResMut<FarmState>) {
    if let Some(active_item) = active_items.items.get_mut(0) {
        active_item.uses -= 1;
        if active_item.uses == 0 {
            active_items.items.pop_front();
        }
    }
}

fn active_items_done(
    active_items: Res<ActiveItems>,
    mut state: ResMut<NextState<FarmingBattleState>>,
) {
    if active_items.items.is_empty() {
        state.set(FarmingBattleState::ShowSummary);
    }
}

fn enter_summary(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.add(ShowDialog {
        handle: asset_server.load("dialogs/basic.yarn"),
        start_node: "FarmingSummary".into(),
    });
}

fn after_summary(
    mut events: EventReader<DialogExited>,
    mut state: ResMut<NextState<FarmingBattleState>>,
) {
    for event in &mut events {
        if &event.node == "FarmingSummary" {
            state.set(FarmingBattleState::DelayTransition);
        }
    }
}

fn back_to_store(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::StoreSetup);
}
