use bevy::math::Vec2;
use bevy::prelude::{Commands, EventReader, KeyCode};
use bevy::prelude::{EventWriter, Query, With};
use iyes_loopless::state::NextState;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::orientation::Direction;
use leafwing_input_manager::Actionlike;

use crate::player::Player;
use crate::GameState;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PlayerAction {
    // movement
    Up,
    Down,
    Left,
    Right,
    // game actions
    Escape,
}

pub struct PlayerMovement {
    pub direction: Vec2,
}

pub struct StateTransition {
    pub next_state: GameState,
}

pub fn on_player_action(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut movement_event: EventWriter<PlayerMovement>,
    mut state_transition_event: EventWriter<StateTransition>,
) {
    let action_state = query.single();

    if action_state.pressed(PlayerAction::Escape) {
        state_transition_event.send(StateTransition {
            next_state: GameState::MainMenu,
        })
    } else {
        let mut direction = Vec2::ZERO;

        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(input_direction) {
                if let Some(movement_direction) = input_direction.direction() {
                    direction += Vec2::from(movement_direction);
                }
            }
        }

        if direction != Vec2::ZERO {
            movement_event.send(PlayerMovement { direction })
        }
    }
}

pub fn on_state_transition(mut events: EventReader<StateTransition>, mut commands: Commands) {
    if let Some(state_transition) = events.iter().next() {
        commands.insert_resource(NextState(state_transition.next_state.clone()));
    }
}

impl PlayerAction {
    const DIRECTIONS: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    pub fn default_input_map() -> InputMap<Self> {
        InputMap::new([
            (KeyCode::Up, Self::Up),
            (KeyCode::Down, Self::Down),
            (KeyCode::Left, Self::Left),
            (KeyCode::Right, Self::Right),
            (KeyCode::W, Self::Up),
            (KeyCode::S, Self::Down),
            (KeyCode::A, Self::Left),
            (KeyCode::D, Self::Right),
            (KeyCode::Escape, Self::Escape),
        ])
    }

    fn direction(self) -> Option<Direction> {
        match self {
            Self::Up => Some(Direction::NORTH),
            Self::Down => Some(Direction::SOUTH),
            Self::Left => Some(Direction::WEST),
            Self::Right => Some(Direction::EAST),
            _ => None,
        }
    }
}
