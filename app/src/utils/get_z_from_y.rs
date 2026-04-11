use crate::constants::STRUCTURE_BASE_Z_INDEX;

/** For depth sorting, derive z from y. Assumes "structure base" map layer. */
pub fn get_z_from_y(y: f32) -> f32 {
  -y / 1000. + STRUCTURE_BASE_Z_INDEX
}