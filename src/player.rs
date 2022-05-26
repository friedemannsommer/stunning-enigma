use bevy::core::Time;
use bevy::prelude::{
    Bundle, Commands, Component, EventReader, OrthographicCameraBundle, Query, Res, Transform, With,
};
use bevy::sprite::SpriteBundle;
use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

use crate::controls::PlayerAction;
use crate::{ImageAssets, PlayerMovement};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    player_actions: InputManagerBundle<PlayerAction>,
    #[bundle]
    sprite: SpriteBundle,
}

pub fn setup_player(mut commands: Commands, images: Res<ImageAssets>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);

    commands.spawn_bundle(PlayerBundle {
        player: Player,
        player_actions: InputManagerBundle {
            input_map: PlayerAction::default_input_map(),
            action_state: ActionState::default(),
        },
        sprite: SpriteBundle {
            texture: images.player.clone(),
            ..Default::default()
        },
    });
}

pub fn on_move_player(
    time: Res<Time>,
    mut events: EventReader<PlayerMovement>,
    mut player_position: Query<&mut Transform, With<Player>>,
) {
    for movement in events.iter() {
        for mut transform in player_position.iter_mut() {
            transform.translation.x += movement.direction.x + 10.0 * time.delta_seconds();
            transform.translation.y += movement.direction.y + 10.0 * time.delta_seconds();
        }
    }
}
