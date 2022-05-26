use bevy::app::AppExit;
use bevy::hierarchy::BuildChildren;
use bevy::math::{Rect, Size};
use bevy::prelude::{
    Button, ButtonBundle, Changed, Color, Commands, Component, EventWriter, NodeBundle, Query, Res,
    TextBundle, With,
};
use bevy::text::{Text, TextStyle};
use bevy::ui::{
    AlignItems, AlignSelf, FlexDirection, Interaction, JustifyContent, Style, UiColor, Val,
};

use crate::{FontAssets, GameState, StateTransition};

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ExitButton;

pub fn setup_menu(mut commands: Commands, fonts: Res<FontAssets>) {
    let title_style = TextStyle {
        font: fonts.kobajeon.clone(),
        font_size: 24.0,
        color: Color::WHITE,
    };
    let button_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: Rect::all(Val::Px(2.0)),
        margin: Rect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..Default::default()
    };

    commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::rgb(0.0, 0.0, 0.0)),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: Rect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|menu| {
            menu.spawn_bundle(ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            })
            .insert(StartButton)
            .with_children(|btn| {
                btn.spawn_bundle(TextBundle {
                    text: Text::with_section("Enter Game", title_style.clone(), Default::default()),
                    ..Default::default()
                });
            });

            menu.spawn_bundle(ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            })
            .insert(ExitButton)
            .with_children(|btn| {
                btn.spawn_bundle(TextBundle {
                    text: Text::with_section("Exit Game", title_style.clone(), Default::default()),
                    ..Default::default()
                });
            });
        });
}

pub fn button_interaction_visual(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = UiColor(Color::rgb(0.5, 0.5, 0.5));
            }
            Interaction::Hovered => {
                *color = UiColor(Color::rgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = UiColor(Color::BLACK);
            }
        }
    }
}

pub fn on_button_interaction<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

pub fn on_exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

pub fn on_start(mut ev: EventWriter<StateTransition>) {
    ev.send(StateTransition {
        next_state: GameState::InGame,
    });
}
