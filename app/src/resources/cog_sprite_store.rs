use bevy::asset::Handle;
use bevy::ecs::resource::Resource;
use bevy::image::{Image, TextureAtlasLayout};

use crate::models::asset_cog_animation_code::AssetCogAnimationCode;
use crate::models::asset_cog_sprite_code::AssetCogSpriteCode;
use crate::models::sprite_map::SpriteMap;

#[derive(Resource)]
pub struct CogSpriteStore {
  pub image_handle: Handle<Image>,
  pub atlas_layout_handle: Handle<TextureAtlasLayout>,
  pub sprite_map: SpriteMap<AssetCogSpriteCode, AssetCogAnimationCode>,
}