use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;

#[derive(Component)]
pub struct UnitAssignedSlot {
  pub target_entity: Entity,
  pub slot_index: u8,
}