use bevy::math::{Rect, Vec3};

use crate::constants::{HALF_TILE_SIZE, TILE_SIZE};

pub fn get_tile_rect(target_translation: &Vec3) -> Rect {
  Rect::new(
    target_translation.x - HALF_TILE_SIZE,
    target_translation.y,
    target_translation.x + HALF_TILE_SIZE,
    target_translation.y + TILE_SIZE,
  )
}