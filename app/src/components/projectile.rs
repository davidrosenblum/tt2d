use bevy::ecs::component::Component;
use uuid::Uuid;

use crate::models::asset_projectile_animation_code::AssetProjectileAnimationCode;
use crate::models::asset_projectile_sprite_code::AssetProjectileSpriteCode;

pub enum ProjectileDirection {
  Left,
  Right,
}

#[derive(Component)]
pub struct Projectile {
  pub speed: f32,
  pub direction: ProjectileDirection,
  pub sprite_code: AssetProjectileSpriteCode,
  // TODO some owner concept (who fired the projectile)
}

impl Projectile {
  /** Does not change, and all projectiles use the same code. */
  pub const ANIMATION_CODE: AssetProjectileAnimationCode = AssetProjectileAnimationCode::Moving;
}