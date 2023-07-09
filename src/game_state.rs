use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<StoreSetupState>()
            .add_system(StoreSetupState::exit_state.in_schedule(OnExit(GameState::StoreSetup)));
        app.add_state::<FarmingBattleState>()
            .add_system(
                FarmingBattleState::enter_state.in_schedule(OnEnter(GameState::FarmingBattle)),
            )
            .add_system(
                FarmingBattleState::exit_state.in_schedule(OnExit(GameState::FarmingBattle)),
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
    // this state exists to avoid a bug with the dialog system
    DelayTransition,
}

impl FarmingBattleState {
    fn enter_state(mut state: ResMut<NextState<FarmingBattleState>>) {
        state.set(FarmingBattleState::CheckSeeded);
    }

    fn exit_state(mut state: ResMut<NextState<FarmingBattleState>>) {
        state.set(FarmingBattleState::Inactive);
    }
}
