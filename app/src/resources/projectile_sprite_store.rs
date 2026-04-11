use bevy::asset::Handle;
use bevy::ecs::resource::Resource;
use bevy::image::{Image, TextureAtlasLayout};

use crate::models::asset_projectile_animation_code::AssetProjectileAnimationCode;
use crate::models::asset_projectile_sprite_code::AssetProjectileSpriteCode;
use crate::models::sprite_map::SpriteMap;

#[derive(Resource)]
pub struct ProjectileSpriteStore {
  pub image_handle: Handle<Image>,
  pub atlas_layout_handle: Handle<TextureAtlasLayout>,
  pub sprite_map: SpriteMap<AssetProjectileSpriteCode, AssetProjectileAnimationCode>,
}