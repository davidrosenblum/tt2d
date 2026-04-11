use bevy::asset::Handle;
use bevy::ecs::resource::Resource;
use bevy::image::{Image, TextureAtlasLayout};

use crate::models::asset_ui_animation_code::AssetUiAnimationCode;
use crate::models::asset_ui_sprite_code::AssetUiSpriteCode;
use crate::models::sprite_map::SpriteMap;

#[derive(Resource)]
pub struct UiSpriteStore {
  pub image_handle: Handle<Image>,
  pub atlas_layout_handle: Handle<TextureAtlasLayout>,
  pub sprite_map: SpriteMap<AssetUiSpriteCode, AssetUiAnimationCode>,
}