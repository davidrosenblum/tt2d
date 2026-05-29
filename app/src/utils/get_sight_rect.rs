use bevy::math::{Rect, Vec3};

use crate::constants::{HALF_TILE_SIZE, TILE_SIZE};

pub fn get_sight_rect(target_translation: &Vec3, sight_radius: f32) -> Rect {
  Rect::new(
    target_translation.x - HALF_TILE_SIZE - sight_radius,
    target_translation.y - sight_radius,
    target_translation.x + HALF_TILE_SIZE + sight_radius,
    target_translation.y + TILE_SIZE + sight_radius,
  )
}