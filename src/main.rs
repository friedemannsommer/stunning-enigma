#![cfg_attr(
    all(not(debug_assertions), target_family = "windows"),
    windows_subsystem = "windows"
)]

use bevy::app::App;
use bevy::ecs::event::Events;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, UiCameraBundle, With};
use bevy::DefaultPlugins;
use bevy_asset_loader::AssetCollectionApp;
use iyes_loopless::condition::{ConditionSet, IntoConditionalSystem};
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::plugin::InputManagerPlugin;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use mimalloc::MiMalloc;

use crate::assets::{FontAssets, ImageAssets};
use crate::controls::{
    on_player_action, on_state_transition, PlayerAction, PlayerMovement, StateTransition,
};
use crate::main_menu::{
    button_interaction_visual, on_button_interaction, on_exit, on_start, setup_menu, ExitButton,
    MainMenu, StartButton,
};
use crate::player::{on_move_player, setup_player, Player, PlayerCamera};
use crate::state::GameState;

mod assets;
mod controls;
mod main_menu;
mod player;
mod state;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let mut app = App::new();

    // plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default());

    // assets
    app.init_collection::<FontAssets>()
        .init_collection::<ImageAssets>();

    // events
    app.add_event::<StateTransition>()
        .add_event::<PlayerMovement>();

    // state
    app.add_loopless_state(GameState::MainMenu);

    // systems
    app.add_enter_system(GameState::MainMenu, setup_menu)
        .add_enter_system(GameState::InGame, setup_player)
        .add_exit_system(GameState::MainMenu, remove_with::<MainMenu>)
        .add_exit_system(GameState::InGame, remove_with::<Player>)
        .add_exit_system(GameState::InGame, remove_with::<PlayerCamera>)
        .add_exit_system(GameState::InGame, clear_events::<PlayerMovement>)
        .add_system(on_state_transition.run_on_event::<StateTransition>())
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

fn remove_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn clear_events<T: 'static + Send + Sync>(mut events: ResMut<Events<T>>) {
    events.clear();
}
