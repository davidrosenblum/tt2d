use bevy::ecs::component::Component;
use uuid::Uuid;

#[derive(Component)]
pub struct Unit {
  pub id: Uuid,
}