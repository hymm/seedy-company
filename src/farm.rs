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
        // GameState::FarmingBattle systems
        app.add_systems(OnEnter(GameState::FarmingBattle), FarmState::spawn_farm)
            .add_systems(
                Update,
                FarmState::update_farm.run_if(in_state(GameState::FarmingBattle)),
            )
            .add_systems(OnExit(GameState::FarmingBattle), FarmState::despawn_farm);

        app.add_systems(OnEnter(FarmingBattleState::CheckSeeded), check_full_grown)
            .add_systems(
                Update,
                check_seeded
                    .run_if(in_state(FarmingBattleState::CheckSeeded))
                    .run_if(on_timer(Duration::from_secs_f32(0.5))),
            );

        // FarmingBattleState::ApplyItems systems
        app.add_systems(
            Update,
            apply_active_item
                .run_if(in_state(FarmingBattleState::ApplyItems))
                .run_if(on_timer(Duration::from_secs_f32(0.5))),
        )
        .add_systems(
            Update,
            active_items_done.run_if(in_state(FarmingBattleState::ApplyItems)),
        );

        app.add_systems(
            Update,
            check_after
                .run_if(in_state(FarmingBattleState::CheckFailed))
                .run_if(on_timer(Duration::from_secs_f32(0.5))),
        );

        app.add_systems(OnEnter(FarmingBattleState::ShowSummary), enter_summary)
            .add_systems(
                Update,
                after_summary.run_if(in_state(FarmingBattleState::ShowSummary)),
            );
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

#[derive(Component)]
struct FarmMarker;

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
        // spawn background
        commands.spawn((
            FarmMarker,
            SpriteBundle {
                texture: asset_server.load("images/Farm_Screen_Spring.png"),
                ..default()
            },
        ));

        // spawn tiles
        let farm_state = if let Some(farm_state) = farm_state {
            farm_state.clone()
        } else {
            FarmState {
                tiles: [FarmTile::Dirt; 25],
            }
        };
        const TILE_SIZE: f32 = 24.;
        let start_pos_x = -TILE_SIZE * 4. + 7.;
        let start_pos_y = -TILE_SIZE * 1. - 11.;
        for i in 0..5 {
            for j in 0..5 {
                let tile = farm_state.tiles[j + i * 5];
                commands.spawn((
                    FarmMarker,
                    tile,
                    SpriteBundle {
                        texture: asset_server.load(tile.get_asset_path()),
                        transform: Transform::from_xyz(
                            start_pos_x + TILE_SIZE * i as f32,
                            start_pos_y + TILE_SIZE * j as f32,
                            1.0,
                        ),
                        ..default()
                    },
                ));
            }
        }

        commands.insert_resource(farm_state);
    }

    fn despawn_farm(mut commands: Commands, q: Query<Entity, With<FarmMarker>>) {
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

fn check_full_grown(mut farm_state: ResMut<FarmState>) {
    for tile in farm_state.tiles.iter_mut() {
        if *tile == FarmTile::SproutedWet {
            *tile = FarmTile::FullGrown;
        }
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
                if let Some(tile) = farm_state.find(FarmTile::FullGrown) {
                    *tile = FarmTile::Dirt;
                } else if let Some(tile) = farm_state.find(FarmTile::Failed) {
                    *tile = FarmTile::Dirt;
                } else {
                    active_items.items.pop_front();
                    return;
                }
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

// transition dry sprouted to failed
fn check_after(
    mut farm_state: ResMut<FarmState>,
    mut state: ResMut<NextState<FarmingBattleState>>,
) {
    // check failed
    if let Some(tile) = farm_state.find(FarmTile::SproutedDry) {
        *tile = FarmTile::Failed;
        return;
    }

    state.set(FarmingBattleState::ShowSummary);
}

fn active_items_done(
    active_items: Res<ActiveItems>,
    mut state: ResMut<NextState<FarmingBattleState>>,
) {
    if active_items.items.is_empty() {
        state.set(FarmingBattleState::CheckFailed);
    }
}

fn enter_summary(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.add(ShowDialog {
        handle: asset_server.load("dialogs/basic.yarn"),
        start_node: "FarmingSummary".into(),
    });
}

fn after_summary(mut events: EventReader<DialogExited>, mut state2: ResMut<NextState<GameState>>) {
    for event in &mut events {
        if &event.node == "FarmingSummary" {
            state2.set(GameState::StoreSetup);
        }
    }
}
