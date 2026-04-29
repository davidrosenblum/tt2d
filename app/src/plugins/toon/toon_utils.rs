use bevy::ecs::bundle::Bundle;
use bevy::image::TextureAtlas;
use bevy::math::Vec2;
use bevy::sprite::{Anchor, Sprite};
use bevy::transform::components::Transform;
use uuid::Uuid;

use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::toon::Toon;
use crate::components::toon_animation::ToonAnimation;
use crate::components::toon_bundle::ToonBundle;
use crate::components::toon_companion::ToonCompanion;
use crate::components::toon_sprite::ToonSprite;
use crate::components::unit::Unit;
use crate::components::unit_facing::UnitFacing;
use crate::components::unit_movement::UnitMovement;
use crate::constants::TOON_SIZE;
use crate::data::toon_companion_data::TOON_COMANION_DATA_STORE;
use crate::data::toon_npc_data::TOON_NPC_DATA_STORE;
use crate::models::asset_toon_animation_code::AssetToonAnimationCode;
use crate::models::asset_toon_sprite_code::AssetToonSpriteCode;
use crate::models::facing_code::FacingCode;
use crate::models::toon_companion_code::ToonCompanionCode;
use crate::models::toon_npc_code::ToonNpcCode;
use crate::resources::toon_sprite_store::ToonSpriteStore;
use crate::utils::get_z_from_y::get_z_from_y;

pub fn build_toon_bundle(
  sprite_code: AssetToonSpriteCode,
  facing_code: FacingCode,
  name: &str,
  position: Vec2,
  toon_sprite_store: &ToonSpriteStore,
) -> Option<ToonBundle> {
  let animation_code = AssetToonAnimationCode::Idle;
  let Some(frame) = toon_sprite_store.sprite_map.get_first_frame(&sprite_code, &animation_code) else {
    return None;
  };

  let mut sprite = Sprite::from_atlas_image(
    toon_sprite_store.image_handle.clone(),
    TextureAtlas {
      index: frame.layout_index,
      layout: toon_sprite_store.atlas_layout_handle.clone(),
    },
  );
  sprite.custom_size = Some(Vec2::splat(TOON_SIZE));

  let toon_bundle = ToonBundle {
    anchor: Anchor::BOTTOM_CENTER,
    sprite,
    sprite_animation_state: SpriteAnimationState::new(frame.duration),
    toon: Toon {
      name: name.into(),
    },
    toon_animation: ToonAnimation(animation_code),
    toon_sprite: ToonSprite(sprite_code),
    transform: Transform::from_xyz(position.x, position.y, get_z_from_y(position.y)),
    unit: Unit {
      id: Uuid::new_v4(),
    },
    unit_facing: UnitFacing(facing_code),
    unit_movement: UnitMovement::new(1.),
  };
  Some(toon_bundle)
}

pub fn build_toon_companion_bundle(
  toon_companion_code: &ToonCompanionCode,
  facing_code: &FacingCode,
  position: Vec2,
  toon_sprite_store: &ToonSpriteStore,
) -> Option<impl Bundle> {
  let companion_data = TOON_COMANION_DATA_STORE.iter().find(|data| data.companion_code == *toon_companion_code)?;
  let bundle = (
    build_toon_bundle(companion_data.sprite_code, *facing_code, companion_data.name, position, toon_sprite_store)?,
    ToonCompanion {
      companion_code: companion_data.companion_code,
    },
  );
  Some(bundle)
}

pub fn build_toon_npc_bundle(
  toon_npc_code: &ToonNpcCode,
  facing_code: &FacingCode,
  position: Vec2,
  toon_sprite_store: &ToonSpriteStore,
) -> Option<impl Bundle> {
  let npc_data = TOON_NPC_DATA_STORE.iter().find(|data| data.npc_code == *toon_npc_code)?;
  let bundle = (
    build_toon_bundle(npc_data.sprite_code, *facing_code, npc_data.name, position, toon_sprite_store)?,
  );
  Some(bundle)
}