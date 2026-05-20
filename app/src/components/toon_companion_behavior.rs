use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;

#[derive(Component)]
pub enum ToonCompanionBehavior {
  FollowPlayer,
  TargetCog(Entity),
}