use bevy::prelude::*;

use crate::game_state::GameState;

pub struct FarmPlugin;
impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_farm.in_schedule(OnEnter(GameState::FarmingBattle)));
    }
}

enum FarmTile {
    Dirt,
    Tilled,
    Seeded,
    Sprouted,
    FullGrown,
}

fn spawn_farm(mut commands: Commands) {
    const TILE_SIZE: f32 = 24.;
    let start_pos = -TILE_SIZE * 2.5;
    for i in 0..5 {
        for j in 0..5 {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(24., 24.)),
                    color: Color::rgb_u8(196, 164, 132),
                    ..default()
                },
                transform: Transform::from_xyz(
                    start_pos + TILE_SIZE * i as f32,
                    start_pos + TILE_SIZE * j as f32,
                    0.0,
                ),
                ..default()
            });
        }
    }
}
