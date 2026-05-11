use bevy::ecs::component::Component;
use bevy::math::Rect;

#[derive(Component)]
pub struct CogRegion {
  pub bounds: Rect,
}