use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UnitFacing {
  Left,
  Right,
}