use bevy::asset::Handle;
use bevy::ecs::resource::Resource;
use bevy::image::Image;

use crate::assets::tiled_map_json::TiledMapJson;
use crate::data::map_data::MapData;
use crate::models::map_code::MapCode;

#[derive(Resource)]
pub struct MapLoadTracker {
  pub map_code: MapCode,
  pub map_data: MapData,
  pub tileset_terrain_image_handle: Handle<Image>,
  pub tileset_structure_image_handle: Handle<Image>,
  pub tilemap_json_handle: Handle<TiledMapJson>,
}