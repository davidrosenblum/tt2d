use bevy::ecs::component::Component;
use bevy::math::Rect;

use crate::models::map_code::MapCode;

#[derive(Component)]
pub struct TeleportRegion {
  pub map_code: MapCode,
  pub bounds: Rect,
}