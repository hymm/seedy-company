use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_state::<StoreSetupState>();
    }
}

#[derive(States, PartialEq, Eq, Default, Debug, Hash, Clone)]
pub enum GameState {
    #[default]
    Start,
    StoreSetup,
    FarmingBattle,
    Failed,
    Success,
}

#[derive(States, PartialEq, Eq, Default, Debug, Hash, Clone)]
pub enum StoreSetupState {
    #[default]
    Inactive,
    OpeningDialog,
    PedestalSelect,
    Inventory,
    PriceSelect,
    FarmerBuy,
}
