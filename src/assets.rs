use bevy::asset::{AssetServer, Handle, LoadState};
use bevy::prelude::{Commands, EventWriter, Image, Res};
use bevy::text::Font;

use crate::{GameState, StateTransition};

pub struct SpriteAssets {
    pub explosions: Vec<Handle<Image>>,
    pub fires: Vec<Handle<Image>>,
    pub ships: Vec<Handle<Image>>,
    pub tiles: Vec<Handle<Image>>,
}

pub struct FontAssets {
    pub kenney_block: Handle<Font>,
}

pub fn load_initial_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontAssets {
        kenney_block: asset_server.load("kenney-fonts/Kenney Blocks.ttf"),
    });
}

pub fn load_game_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut tiles = Vec::<Handle<Image>>::with_capacity(96);
    let mut ships = Vec::<Handle<Image>>::with_capacity(24);
    let mut explosions = Vec::<Handle<Image>>::with_capacity(3);
    let mut fires = Vec::<Handle<Image>>::with_capacity(2);

    for i in 1..97 {
        tiles.push(asset_server.load(&format!("kenney-pirate-pack/Tiles/tile_{:0>2}.png", i)));
    }

    for i in 1..25 {
        ships.push(asset_server.load(&format!("kenney-pirate-pack/Ships/ship ({}).png", i)));
    }

    for i in 1..4 {
        explosions
            .push(asset_server.load(&format!("kenney-pirate-pack/Effects/explosion{}.png", i)));
    }

    for i in 1..3 {
        fires.push(asset_server.load(&format!("kenney-pirate-pack/Effects/fire{}.png", i)));
    }

    commands.insert_resource(SpriteAssets {
        explosions,
        fires,
        ships,
        tiles,
    })
}

pub fn initial_load_transition(
    mut state_event: EventWriter<StateTransition>,
    fonts: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    if asset_server.get_load_state(&fonts.kenney_block) == LoadState::Loaded {
        state_event.send(StateTransition {
            next_state: GameState::MainMenu,
        });
    }
}

pub fn game_asset_load_transition(
    mut state_event: EventWriter<StateTransition>,
    sprites: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
) {
    if asset_server.get_group_load_state(sprites.explosions.iter().map(|h| h.id))
        == LoadState::Loaded
        && asset_server.get_group_load_state(sprites.fires.iter().map(|h| h.id))
            == LoadState::Loaded
        && asset_server.get_group_load_state(sprites.ships.iter().map(|h| h.id))
            == LoadState::Loaded
        && asset_server.get_group_load_state(sprites.tiles.iter().map(|h| h.id))
            == LoadState::Loaded
    {
        state_event.send(StateTransition {
            next_state: GameState::InGame,
        });
    }
}
