use bevy::ecs::component::Component;

use crate::models::toon_companion_code::ToonCompanionCode;

#[derive(Component)]
pub struct ToonCompanion {
  pub companion_code: ToonCompanionCode,
}