use bevy::ecs::resource::Resource;
use bevy::math::Rect;

use crate::data::map_data::MapData;
use crate::models::map_code::MapCode;
use crate::models::map_hub_code::MapHubCode;

#[derive(Resource)]
pub struct MapContext {
  pub map_code: MapCode,
  pub map_data: MapData,
  pub bounds: Rect,
  pub prev_map_hub_code: Option<MapHubCode>,
}