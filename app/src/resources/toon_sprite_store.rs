use bevy::asset::Handle;
use bevy::ecs::resource::Resource;
use bevy::image::{Image, TextureAtlasLayout};

use crate::models::asset_toon_animation_code::AssetToonAnimationCode;
use crate::models::asset_toon_sprite_code::AssetToonSpriteCode;
use crate::models::sprite_map::SpriteMap;

#[derive(Resource)]
pub struct ToonSpriteStore {
  pub image_handle: Handle<Image>,
  pub atlas_layout_handle: Handle<TextureAtlasLayout>,
  pub sprite_map: SpriteMap<AssetToonSpriteCode, AssetToonAnimationCode>,
}