use bevy::ecs::bundle::Bundle;
use bevy::sprite::{Anchor, Sprite};
use bevy::transform::components::Transform;

use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::toon::Toon;
use crate::components::toon_animation::ToonAnimation;
use crate::components::toon_sprite::ToonSprite;
use crate::components::unit::Unit;
use crate::components::unit_facing::UnitFacing;
use crate::components::unit_movement::UnitMovement;

#[derive(Bundle)]
pub struct ToonBundle {
  pub anchor: Anchor,
  pub toon: Toon,
  pub toon_animation: ToonAnimation,
  pub toon_sprite: ToonSprite,
  pub sprite: Sprite,
  pub sprite_animation_state: SpriteAnimationState,
  pub transform: Transform,
  pub unit: Unit,
  pub unit_facing: UnitFacing,
  pub unit_movement: UnitMovement,
}