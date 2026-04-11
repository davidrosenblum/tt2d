use bevy::ecs::component::Component;
use bevy::time::{Timer, TimerMode};

#[derive(Component)]
pub struct SpriteAnimationState {
  pub frame_index: usize,
  pub timer: Timer,
}

impl SpriteAnimationState {
  pub fn new(duration: u32) -> Self {
    Self {
      frame_index: 0,
      timer: Timer::from_seconds(duration as f32 / 1000., TimerMode::Repeating),
    }
  }
}