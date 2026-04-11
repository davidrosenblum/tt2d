use std::hash::Hash;

use bevy::platform::collections::HashMap;

use crate::models::sprite_animation_frame_data::SpriteAnimationFrameData;

pub struct SpriteMap<S, A> {
  map: HashMap<S, HashMap<A, Vec<SpriteAnimationFrameData>>>,
}

impl<S: Eq + Hash, A: Eq + Hash> SpriteMap<S, A> {
  pub fn new(map: HashMap<S, HashMap<A, Vec<SpriteAnimationFrameData>>>) -> Self {
    Self {
      map,
    }
  }

  pub fn get_frames(&self, sprite_code: &S, animation_code: &A) -> Option<&Vec<SpriteAnimationFrameData>> {
    self.map.get(sprite_code).and_then(|animations| animations.get(animation_code))
  }

  pub fn get_first_frame(&self, sprite_code: &S, animation_code: &A) -> Option<&SpriteAnimationFrameData> {
    self.get_frames(sprite_code, animation_code).and_then(|frames| frames.first())
  }

  pub fn get_first_frame_index(&self, sprite_code: &S, animation_code: &A) -> Option<usize> {
    self.get_first_frame(sprite_code, animation_code).and_then(|frame| Some(frame.layout_index))
  }
}