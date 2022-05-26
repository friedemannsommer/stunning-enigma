use bevy::asset::{AssetServer, Handle, HandleUntyped};
use bevy::prelude::{Image, World};
use bevy::text::Font;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "icon.png")]
    pub player: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "kobajeon-font/kobajeon.otf")]
    pub kobajeon: Handle<Font>,
}
