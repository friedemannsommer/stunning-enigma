#![cfg_attr(
    all(not(debug_assertions), target_family = "windows"),
    windows_subsystem = "windows"
)]

use bevy::{
    app::{App, PluginGroup},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    DefaultPlugins,
};
use iyes_loopless::{
    condition::{ConditionSet, IntoConditionalSystem},
    prelude::AppLooplessStateExt,
};
use leafwing_input_manager::plugin::InputManagerPlugin;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use mimalloc::MiMalloc;

use crate::{
    assets::{
        game_asset_load_transition, initial_load_transition, load_game_assets, load_initial_assets,
    },
    controls::{
        on_player_action, on_state_transition, PlayerAction, PlayerMovement, StateTransition,
    },
    player::{move_camera_to_player, on_move_player, setup_player, Player, PlayerCamera},
    scenes::{
        in_game::{setup_in_game, Tile},
        loading::{setup_loading, Loading},
        main_menu::{
            button_interaction_visual, on_button_interaction, on_exit, on_start, setup_menu,
            ExitButton, MainMenu, StartButton,
        },
    },
    state::GameState,
    utils::{clear_events, remove_with},
};

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
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    // plugins
    app.add_plugins(DefaultPlugins.set(bevy::window::WindowPlugin {
        window: bevy::window::WindowDescriptor {
            fit_canvas_to_parent: true,
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            resizable: true,
            title: String::from("Stunning Enigma"),
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_plugin(InputManagerPlugin::<PlayerAction>::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(LogDiagnosticsPlugin::default());

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
        .add_enter_system(GameState::InGame, setup_in_game)
        .add_exit_system(GameState::MainMenu, remove_with::<MainMenu>)
        .add_exit_system(GameState::InGame, remove_with::<Player>)
        .add_exit_system(GameState::InGame, remove_with::<PlayerCamera>)
        .add_exit_system(GameState::InGame, remove_with::<Tile>)
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
                .with_system(move_camera_to_player)
                .into(),
        );

    // launch!
    app.run();
}
