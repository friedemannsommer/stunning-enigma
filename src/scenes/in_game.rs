use bevy::prelude::{Commands, Component, Res, Transform};
use bevy::sprite::SpriteBundle;

use crate::assets::SpriteAssets;

#[derive(Component, Debug)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub variant: TileVariant,
}

#[derive(Clone, Debug)]
pub enum TileVariant {
    Water = 72,
}

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const TILE_SIZE: u32 = 64;

pub fn setup_in_game(mut commands: Commands, images: Res<SpriteAssets>) {
    for tile in generate_world(WIDTH, HEIGHT) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: images.tiles[tile.variant.clone() as usize].clone(),
                transform: Transform::from_xyz(tile.x, tile.y, 0.0),
                ..Default::default()
            })
            .insert(tile);
    }
}

fn generate_world(width: u32, height: u32) -> Vec<Tile> {
    let mut tiles = Vec::<Tile>::with_capacity(width.saturating_mul(height) as usize);
    let half_width = ((width / 2) * TILE_SIZE) as f32;
    let half_height = ((height / 2) * TILE_SIZE) as f32;

    for x in 0..width {
        for y in 0..height {
            tiles.push(Tile {
                x: (x * TILE_SIZE) as f32 - half_width,
                y: (y * TILE_SIZE) as f32 - half_height,
                variant: TileVariant::Water,
            });
        }
    }

    tiles
}
