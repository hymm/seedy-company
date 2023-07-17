use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<StoreSetupState>()
            .add_systems(OnExit(GameState::StoreSetup), StoreSetupState::exit_state);
        app.add_state::<FarmingBattleState>()
            .add_systems(
                OnEnter(GameState::FarmingBattle),
                FarmingBattleState::enter_state,
            )
            .add_systems(
                OnExit(GameState::FarmingBattle),
                FarmingBattleState::exit_state,
            );
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

impl StoreSetupState {
    fn exit_state(mut state: ResMut<NextState<StoreSetupState>>) {
        state.set(StoreSetupState::Inactive);
    }
}

#[derive(States, PartialEq, Eq, Default, Debug, Hash, Clone)]
pub enum FarmingBattleState {
    #[default]
    Inactive,
    CheckSeeded,
    ApplyItems,
    CheckFailed,
    ShowSummary,
}

impl FarmingBattleState {
    fn enter_state(mut state: ResMut<NextState<FarmingBattleState>>) {
        state.set(FarmingBattleState::CheckSeeded);
    }

    fn exit_state(mut state: ResMut<NextState<FarmingBattleState>>) {
        state.set(FarmingBattleState::Inactive);
    }
}
