use bevy::asset::Handle;
use bevy::ecs::resource::Resource;
use bevy::image::Image;
use bevy::platform::collections::HashMap;

use crate::assets::aseprite_json::AsepriteJson;

/** Keeps handles alive, intended for assets that are never unloaded. */
#[derive(Default, Resource)]
pub struct GlobalAssetStore {
  pub image_handles: HashMap<String, Handle<Image>>,
  pub animation_handles: HashMap<String, Handle<AsepriteJson>>,
}