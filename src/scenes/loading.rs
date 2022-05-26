use bevy::hierarchy::BuildChildren;
use bevy::math::{Rect, Size};
use bevy::prelude::{Color, Commands, Component, NodeBundle, Res, TextBundle};
use bevy::text::{Text, TextStyle};
use bevy::ui::{AlignSelf, FlexDirection, JustifyContent, Style, UiColor, Val};

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
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::BLACK),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: Rect::all(Val::Auto),
                padding: Rect::all(Val::Px(24.0)),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Loading)
        .with_children(|menu| {
            menu.spawn_bundle(TextBundle {
                text: Text::with_section("Loading...", title_style.clone(), Default::default()),
                ..Default::default()
            });
        });
}
