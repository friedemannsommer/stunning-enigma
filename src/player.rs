use bevy::core::Time;
use bevy::math::Quat;
use bevy::prelude::{
    Bundle, Changed, Commands, Component, EventReader, OrthographicCameraBundle, Query, Res,
    Transform, With, Without,
};
use bevy::sprite::SpriteBundle;
use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

use crate::assets::SpriteAssets;
use crate::controls::{PlayerAction, PlayerMovement};

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

pub fn setup_player(mut commands: Commands, images: Res<SpriteAssets>) {
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
            texture: images.ships[0].clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
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
        let velocity = movement.direction.normalize() * 100.0 * time.delta_seconds();

        for mut transform in player_position.iter_mut() {
            transform.translation.x += velocity.x;
            transform.translation.y += velocity.y;
            transform.rotation = Quat::from_rotation_z(velocity.x.atan2(-velocity.y));
        }
    }
}

pub fn move_camera_to_player(
    player_position: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_position: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    if !player_position.is_empty() {
        let mut camera_transform = camera_position.single_mut();
        let player_transform = player_position.single();

        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
