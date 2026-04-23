use bevy::image::TextureAtlas;
use bevy::math::Vec2;
use bevy::sprite::{Anchor, Sprite};
use bevy::transform::components::Transform;
use uuid::Uuid;

use crate::components::cog::Cog;
use crate::components::cog_animation::CogAnimation;
use crate::components::cog_bundle::CogBundle;
use crate::components::cog_sprite::CogSprite;
use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::unit::Unit;
use crate::components::unit_facing::UnitFacing;
use crate::components::unit_movement::UnitMovement;
use crate::constants::COG_SIZE;
use crate::data::cog_data::{COG_DATA_STORE, CogData};
use crate::models::asset_cog_animation_code::AssetCogAnimationCode;
use crate::models::cog_department_code::CogDepartmentCode;
use crate::models::facing_code::FacingCode;
use crate::resources::cog_sprite_store::CogSpriteStore;
use crate::utils::get_z_from_y::get_z_from_y;

pub fn build_cog_bundle(
  cog_data: &CogData,
  facing_code: FacingCode,
  position: Vec2,
  cog_sprite_store: &CogSpriteStore,
) -> Option<CogBundle> {
  let animation_code = AssetCogAnimationCode::Idle;

  let Some(frame) = cog_sprite_store.sprite_map.get_first_frame(&cog_data.sprite_code, &animation_code) else {
    return None;
  };

  let mut sprite = Sprite::from_atlas_image(
    cog_sprite_store.image_handle.clone(),
    TextureAtlas {
      index: frame.layout_index,
      layout: cog_sprite_store.atlas_layout_handle.clone(),
    },
  );
  sprite.custom_size = Some(Vec2::splat(COG_SIZE));

  let cog_bundle = CogBundle {
    anchor: Anchor::BOTTOM_CENTER,
    cog: Cog {
      name: cog_data.name.to_string(),
      department_code: cog_data.department_code,
    },
    cog_animation: CogAnimation(animation_code),
    cog_sprite: CogSprite(cog_data.sprite_code),
    sprite,
    sprite_animation_state: SpriteAnimationState::new(frame.duration),
    transform: Transform::from_xyz(position.x, position.y, get_z_from_y(position.y)),
    unit: Unit {
      id: Uuid::new_v4(),
    },
    unit_facing: UnitFacing(facing_code),
    unit_movement: UnitMovement::new(1.),
  };
  Some(cog_bundle)
}

pub fn build_cog_bundle_from_dept_tier(
  cog_department_code: CogDepartmentCode,
  facing_code: FacingCode,
  tier: u32,
  position: Vec2,
  cog_sprite_store: &CogSpriteStore,
) -> Option<CogBundle> {
  let Some(cog_data) = COG_DATA_STORE.iter().find(|cog_data| {
    cog_data.department_code == cog_department_code && cog_data.tier == tier
  }) else {
    return None;
  };
  build_cog_bundle(cog_data, facing_code, position, cog_sprite_store)
}