use bevy::app::{Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageReader;
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Commands, If, Query, Res};
use bevy::sprite::Sprite;
use bevy::time::Time;
use bevy::transform::components::Transform;

use crate::components::cog::Cog;
use crate::components::cog_animation::CogAnimation;
use crate::components::cog_sprite::CogSprite;
use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::unit_facing::UnitFacing;
use crate::messages::load_map_requested::LoadMapRequested;
use crate::messages::loaded_map::LoadedMap;
use crate::models::facing_code::FacingCode;
use crate::plugins::cog::cog_utils::process_map_object;
use crate::resources::cog_sprite_store::CogSpriteStore;

pub struct CogPlugin;

impl Plugin for CogPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(Update, (update_cog_facing_direction, update_cog_animation_frame));
    app.add_systems(Update, (poll_map_load_requested, poll_map_loaded));
  }
}

fn update_cog_animation_frame(
  cogs_query: Query<(&CogSprite, &CogAnimation, &mut SpriteAnimationState, &mut Sprite), With<Cog>>,
  time: Res<Time>,
  cog_sprite_store: If<Res<CogSpriteStore>>,
) {
  let delta = time.delta();
  for (cog_sprite, cog_animation, mut spritesheet_animation_state, mut sprite) in cogs_query {
    spritesheet_animation_state.timer.tick(delta);

    if !spritesheet_animation_state.timer.just_finished() {
      continue;
    }

    let Some(frames) = cog_sprite_store.sprite_map.get_frames(cog_sprite, cog_animation) else {
      continue;
    };

    let mut next_frame_index = spritesheet_animation_state.frame_index + 1;
    if next_frame_index >= frames.len() {
      next_frame_index = 0;
    }
    spritesheet_animation_state.frame_index = next_frame_index;

    if let Some(texture_atlas) = &mut sprite.texture_atlas {
      if let Some(layout_index) = frames.get(next_frame_index).and_then(|frame| Some(frame.layout_index)) {
        texture_atlas.index = layout_index;
      }
    }
  }
}

fn update_cog_facing_direction(
  cogs_query: Query<(&UnitFacing, &mut Transform), (With<Cog>, Changed<UnitFacing>)>,
) {
  for (unit_facing, mut transform) in cogs_query {
    transform.scale.x = match **unit_facing {
      FacingCode::Left => -1.,
      FacingCode::Right => 1.,
    };
  }
}

// TODO move cogs to player if in range
// TODO cogs face place if in range

fn poll_map_load_requested(
  mut commands: Commands,
  mut message_reader: MessageReader<LoadMapRequested>,
  cogs_query: Query<Entity, With<Cog>>,
) {
  if message_reader.is_empty() {
    return;
  }
  message_reader.clear();

  for cog_entity in cogs_query {
    commands.entity(cog_entity).despawn();
  }
}

fn poll_map_loaded(
  mut commands: Commands,
  mut message_reader: MessageReader<LoadedMap>,
  cog_sprite_store: If<Res<CogSpriteStore>>,
) {
  let Some(message) = message_reader.read().last() else {
    return;
  };
  
  for map_object in &message.map_objects {
    process_map_object(
      &mut commands,
      map_object,
      &cog_sprite_store,
      message.tilewidth,
      message.tileheight,
      message.height,
    );
  }
}

// TODO cog region
// Need a region concept that 1) spawns cogs randomly and 2) attaches cogs to the region to not leave its bounds
// Where should the region Rect live? Is that a resource?
// Also, update MapData to have all the maps