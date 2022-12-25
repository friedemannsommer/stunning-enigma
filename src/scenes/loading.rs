use bevy::{
    hierarchy::BuildChildren,
    prelude::{Camera2dBundle, Color, Commands, Component, NodeBundle, Res, TextBundle},
    text::{Text, TextStyle},
    ui::{AlignSelf, BackgroundColor, FlexDirection, JustifyContent, Size, Style, UiRect, Val},
};

use crate::assets::FontAssets;

#[derive(Component)]
pub struct Loading;

pub fn setup_loading(mut commands: Commands, fonts: Res<FontAssets>) {
    let title_style = TextStyle {
        font: fonts.kenney_block.clone(),
        font_size: 48.0,
        color: Color::WHITE,
    };

    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor::from(Color::BLACK),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                padding: UiRect::all(Val::Px(24.0)),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Loading)
        .with_children(|menu| {
            menu.spawn(TextBundle {
                text: Text::from_section("Loading...", title_style.clone()),
                ..Default::default()
            });
        });

    commands.spawn(Camera2dBundle::default()).insert(Loading);
}
