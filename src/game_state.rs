use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
          .add_state::<RunningState>();
    }
}

#[derive(States, PartialEq, Eq, Default, Debug, Hash, Clone)]
pub enum GameState {
  #[default]
  Start,
  Running,
  Failed,
  Success,
}

#[derive(States, PartialEq, Eq, Default, Debug, Hash, Clone)]
pub enum RunningState {
  #[default]
  Tutorial,
  Dialog,
  Inventory,
}