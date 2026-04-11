use bevy::math::Vec2;

use crate::constants::TILE_SIZE;

/** Handles converting Tiled object x/y to bevy x/y. */
pub fn normalize_tiled_point(point: &Vec2, tilewidth: u32, tileheight: u32, height: u32) -> Vec2 {
  let scale_x = TILE_SIZE / tilewidth as f32;
  let scale_y = TILE_SIZE / tileheight as f32;

  let map_height = TILE_SIZE * height as f32;

  let x = point.x * scale_x;
  let y = point.y * scale_y;
  let bevy_y = map_height - y;
  Vec2::new(x, bevy_y)
}