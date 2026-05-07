use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CombatHealth {
  pub current: f32,
  pub max: f32,
}

impl CombatHealth {
  pub fn new(max: f32) -> Self {
    Self {
      current: max,
      max,
    }
  }
}