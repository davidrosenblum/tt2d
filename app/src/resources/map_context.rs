use bevy::ecs::resource::Resource;
use bevy::math::Rect;

use crate::data::map_data::MapData;
use crate::models::map_code::MapCode;

#[derive(Resource)]
pub struct MapContext {
  pub map_code: MapCode,
  pub map_data: MapData,
  pub bounds: Rect,
}