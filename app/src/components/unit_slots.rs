use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;

#[derive(Component)]
pub struct UnitSlots {
  pub slots: [Option<Entity>; 8],
}

impl Default for UnitSlots {
  fn default() -> Self {
    Self {
      slots: [None; 8],
    }
  }
}