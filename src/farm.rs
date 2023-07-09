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
            .add_system(FarmState::update_farm.run_if(in_state(GameState::FarmingBattle)))
            .add_system(
                check_seeded
                    .run_if(in_state(FarmingBattleState::CheckSeeded))
                    .run_if(on_timer(Duration::from_secs_f32(0.5))),
            )
            .add_system(
                apply_active_item
                    .run_if(in_state(FarmingBattleState::ApplyItems))
                    .run_if(on_timer(Duration::from_secs_f32(0.5))),
            )
            .add_system(active_items_done.run_if(in_state(FarmingBattleState::ApplyItems)))
            .add_system(enter_summary.in_schedule(OnEnter(FarmingBattleState::ShowSummary)))
            .add_system(after_summary.run_if(in_state(FarmingBattleState::ShowSummary)))
            .add_system(FarmState::despawn_farm.in_schedule(OnExit(GameState::FarmingBattle)))
            .add_system(back_to_store.in_schedule(OnEnter(FarmingBattleState::DelayTransition)));
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
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
            FarmTile::SproutedWet => "images/Watered_Sprout_Tile.png",
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

    fn update_farm(
        mut q: Query<&mut Handle<Image>, With<FarmTile>>,
        farm_state: Res<FarmState>,
        asset_server: Res<AssetServer>,
    ) {
        for (index, mut handle) in q.iter_mut().enumerate() {
            *handle = asset_server.load(farm_state.tiles[index].get_asset_path());
        }
    }

    fn find(&mut self, value: FarmTile) -> Option<&mut FarmTile> {
        self.tiles.iter_mut().find(|tile| **tile == value)
    }
}

// transition seeded to sprouted
fn check_seeded(
    mut farm_state: ResMut<FarmState>,
    mut state: ResMut<NextState<FarmingBattleState>>,
) {
    let Some(tile) = farm_state.find(FarmTile::Seeded) else {
      state.set(FarmingBattleState::ApplyItems);
      return;
    };
    *tile = FarmTile::SproutedDry;
}

fn apply_active_item(mut active_items: ResMut<ActiveItems>, mut farm_state: ResMut<FarmState>) {
    if let Some(active_item) = active_items.items.get_mut(0) {
        match active_item.item_type {
            crate::inventory::ItemType::Hoe => {
                let Some(tile) = farm_state.find(FarmTile::Dirt) else {
                  active_items.items.pop_front();
                  return;
                };
                *tile = FarmTile::Tilled;
            }
            crate::inventory::ItemType::WateringCan => {
                let Some(tile) = farm_state.find(FarmTile::SproutedDry) else {
                active_items.items.pop_front();
                return;
              };
                *tile = FarmTile::SproutedWet;
            }
            crate::inventory::ItemType::Scythe => {
                let Some(tile) = farm_state.find(FarmTile::FullGrown) else {
                active_items.items.pop_front();
                return;
              };
                *tile = FarmTile::Dirt;
            }
            crate::inventory::ItemType::ParsnipSeed => {
                let Some(tile) = farm_state.find(FarmTile::Tilled) else {
                active_items.items.pop_front();
                return;
              };
                *tile = FarmTile::Seeded;
            }
            crate::inventory::ItemType::BlueberrySeed => {
                let Some(tile) = farm_state.find(FarmTile::Tilled) else {
                active_items.items.pop_front();
                return;
              };
                *tile = FarmTile::Seeded;
            }
        }
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
