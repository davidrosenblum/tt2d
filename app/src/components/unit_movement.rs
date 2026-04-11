use bevy::ecs::component::Component;

#[derive(Component)]
pub struct UnitMovement {
  pub speed: f32,
  pub is_movement_enabled: bool,
  pub is_collision_enabled: bool,
}

impl UnitMovement {
  pub fn new(speed: f32) -> Self {
    Self {
      speed,
      is_movement_enabled: true,
      is_collision_enabled: true,
    }
  }
}