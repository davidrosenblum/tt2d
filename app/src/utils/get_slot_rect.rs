use bevy::math::Rect;

use crate::constants::TILE_SIZE;

pub fn get_slot_rect(target_tile_rect: &Rect, slot_index: u8) -> Rect {
  // Remember that target_transaltion is bottomCenter anchored
  let center = target_tile_rect.center();

  let left = target_tile_rect.min.x - TILE_SIZE;
  let center_x = center.x - TILE_SIZE / 2.;
  let right = target_tile_rect.max.x;

  let bottom = target_tile_rect.min.y - TILE_SIZE;
  let center_y = center.y - TILE_SIZE / 2.;
  let top = target_tile_rect.max.y;

  let (x, y) = match slot_index {
    // Top left
    0 => (left, top),
    // Top center
    1 => (center_x, top),
    // Top right
    2 => (right, top),
    // Center left
    3 => (left, center_y),
    // Center right
    4 => (right, center_y),
    // Bottom left
    5 => (left, bottom),
    // Bottom center
    6 => (center_x, bottom),
    // Bottom right
    7 => (right, bottom),
    // Unexpected, default to center
    _ => (center_x, center_y)
  };

  Rect::new(x, y, x + TILE_SIZE, y + TILE_SIZE)
}