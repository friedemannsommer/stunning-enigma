use bevy::{
    app::AppExit,
    hierarchy::BuildChildren,
    prelude::{
        Button, ButtonBundle, Camera2dBundle, Changed, Color, Commands, Component, EventWriter,
        NodeBundle, Query, Res, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, AlignSelf, BackgroundColor, FlexDirection, Interaction, JustifyContent, Size,
        Style, UiRect, Val,
    },
};

use crate::{assets::FontAssets, GameState, StateTransition};

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ExitButton;

pub fn setup_menu(mut commands: Commands, fonts: Res<FontAssets>) {
    let title_style = TextStyle {
        font: fonts.kenney_block.clone(),
        font_size: 48.0,
        color: Color::WHITE,
    };
    let button_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(2.0)),
        margin: UiRect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..Default::default()
    };

    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|menu| {
            menu.spawn(ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            })
            .insert(ExitButton)
            .with_children(|btn| {
                btn.spawn(TextBundle {
                    text: Text::from_section("Exit Game", title_style.clone()),
                    ..Default::default()
                });
            });

            menu.spawn(ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            })
            .insert(StartButton)
            .with_children(|btn| {
                btn.spawn(TextBundle {
                    text: Text::from_section("Enter Game", title_style.clone()),
                    ..Default::default()
                });
            });
        });

    commands.spawn(Camera2dBundle::default()).insert(MainMenu);
}

#[allow(clippy::type_complexity)]
pub fn button_interaction_visual(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(Color::rgb(0.5, 0.5, 0.5));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::BLACK);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
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
        next_state: GameState::Loading,
    });
}
