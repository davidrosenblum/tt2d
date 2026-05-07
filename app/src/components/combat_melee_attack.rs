use bevy::ecs::component::Component;
use bevy::time::{Timer, TimerMode};

#[derive(Component)]
pub struct CombatMeleeAttack {
  pub damage_range: (f32, f32),
  pub cooldown: f32,
  pub cooldown_timer: Timer,
}

impl CombatMeleeAttack {
  pub fn new(cooldown: f32, damage_range: (f32, f32)) -> Self {
    let mut cooldown_timer = Timer::from_seconds(cooldown, TimerMode::Once);
    cooldown_timer.finish();

    Self {
      damage_range,
      cooldown,
      cooldown_timer,
    }
  }
}