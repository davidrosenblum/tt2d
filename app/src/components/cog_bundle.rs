use bevy::ecs::bundle::Bundle;
use bevy::sprite::{Anchor, Sprite};
use bevy::transform::components::Transform;

use crate::components::cog::Cog;
use crate::components::cog_animation::CogAnimation;
use crate::components::cog_behavior::CogBehavior;
use crate::components::cog_spawn_point::CogSpawnPoint;
use crate::components::cog_sprite::CogSprite;
use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::unit::Unit;
use crate::components::unit_facing::UnitFacing;
use crate::components::unit_movement::UnitMovement;
use crate::components::unit_slots::UnitSlots;

#[derive(Bundle)]
pub struct CogBundle {
  pub anchor: Anchor,
  pub cog: Cog,
  pub cog_animation: CogAnimation,
  pub cog_behavior: CogBehavior,
  pub cog_spawn_point: CogSpawnPoint,
  pub cog_sprite: CogSprite,
  pub sprite: Sprite,
  pub sprite_animation_state: SpriteAnimationState,
  pub transform: Transform,
  pub unit: Unit,
  pub unit_facing: UnitFacing,
  pub unit_movement: UnitMovement,
  pub unit_slots: UnitSlots,
}