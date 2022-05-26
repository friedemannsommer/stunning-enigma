use bevy::ecs::event::Events;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, With};

pub fn remove_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn clear_events<T: 'static + Send + Sync>(mut events: ResMut<Events<T>>) {
    events.clear();
}
