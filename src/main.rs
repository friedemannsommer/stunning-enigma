#![cfg_attr(
    all(not(debug_assertions), target_family = "windows"),
    windows_subsystem = "windows"
)]

use bevy::app::App;
use bevy::prelude::{Commands, UiCameraBundle};
use bevy::DefaultPlugins;
use iyes_loopless::condition::{ConditionSet, IntoConditionalSystem};
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::plugin::InputManagerPlugin;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use mimalloc::MiMalloc;

use crate::assets::{
    game_asset_load_transition, initial_load_transition, load_game_assets, load_initial_assets,
};
use crate::controls::{
    on_player_action, on_state_transition, PlayerAction, PlayerMovement, StateTransition,
};
use crate::player::{on_move_player, setup_player, Player, PlayerCamera};
use crate::scenes::loading::{setup_loading, Loading};
use crate::scenes::main_menu::{
    button_interaction_visual, on_button_interaction, on_exit, on_start, setup_menu, ExitButton,
    MainMenu, StartButton,
};
use crate::state::GameState;
use crate::utils::{clear_events, remove_with};

mod assets;
mod controls;
mod player;
mod scenes;
mod state;
mod utils;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let mut app = App::new();

    // plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default());

    // events
    app.add_event::<StateTransition>()
        .add_event::<PlayerMovement>();

    // state
    app.add_loopless_state(GameState::StartUp);

    // systems
    app.add_enter_system(GameState::StartUp, load_initial_assets)
        .add_enter_system(GameState::MainMenu, setup_menu)
        .add_enter_system(GameState::Loading, load_game_assets)
        .add_enter_system(GameState::Loading, setup_loading)
        .add_enter_system(GameState::InGame, setup_player)
        .add_exit_system(GameState::MainMenu, remove_with::<MainMenu>)
        .add_exit_system(GameState::InGame, remove_with::<Player>)
        .add_exit_system(GameState::InGame, remove_with::<PlayerCamera>)
        .add_exit_system(GameState::Loading, remove_with::<Loading>)
        .add_exit_system(GameState::InGame, clear_events::<PlayerMovement>)
        .add_system(on_state_transition.run_on_event::<StateTransition>())
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::StartUp)
                .with_system(initial_load_transition)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Loading)
                .with_system(game_asset_load_transition)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(button_interaction_visual)
                .with_system(on_exit.run_if(on_button_interaction::<ExitButton>))
                .with_system(on_start.run_if(on_button_interaction::<StartButton>))
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(on_player_action)
                .with_system(on_move_player.run_on_event::<PlayerMovement>())
                .into(),
        )
        .add_startup_system(setup);

    // launch!
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
