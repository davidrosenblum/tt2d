use bevy::app::{Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageReader;
use bevy::ecs::query::{Changed, With, Without};
use bevy::ecs::system::{Commands, If, Query, Res};
use bevy::math::Rect;
use bevy::sprite::Sprite;
use bevy::time::Time;
use bevy::transform::components::Transform;

use crate::components::cog::Cog;
use crate::components::cog_animation::CogAnimation;
use crate::components::cog_region::CogRegion;
use crate::components::cog_region_member::CogRegionMember;
use crate::components::cog_sprite::CogSprite;
use crate::components::combat_health::CombatHealth;
use crate::components::combat_target::CombatTarget;
use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::toon::Toon;
use crate::components::unit_facing::UnitFacing;
use crate::components::unit_movement::UnitMovement;
use crate::constants::{COG_SIZE, TILE_SIZE, TOON_SIZE};
use crate::messages::load_map_requested::LoadMapRequested;
use crate::messages::loaded_map::LoadedMap;
use crate::models::asset_cog_animation_code::AssetCogAnimationCode;
use crate::models::facing_code::FacingCode;
use crate::plugins::cog::cog_utils::process_map_object;
use crate::resources::cog_sprite_store::CogSpriteStore;
use crate::utils::get_z_from_y::get_z_from_y;

pub struct CogPlugin;

impl Plugin for CogPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(Update, (update_cog_facing_direction, update_cog_animation_frame));
    app.add_systems(Update, (update_cogs_acquire_target, update_cogs_follow_target));
    app.add_systems(Update, (poll_map_load_requested, poll_map_loaded));
  }
}

const SIGHT_RANGE: f32 = TILE_SIZE * 2.;
const HALF_COG_SIZE: f32 = COG_SIZE / 2.;
const HALF_TOON_SIZE: f32 = TOON_SIZE / 2.;

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

fn update_cogs_acquire_target(
  mut commands: Commands,
  cogs_query: Query<(Entity, &Transform), (With<Cog>, With<CombatHealth>, Without<CombatTarget>, Without<Toon>)>,
  toons_query: Query<(Entity, &Transform), (With<Toon>, With<CombatHealth>, Without<Cog>)>,
) {
  for (cog_entity, cog_transform) in cogs_query {
    let cog_sight_rect = Rect::new(
      cog_transform.translation.x - HALF_COG_SIZE - SIGHT_RANGE,
      cog_transform.translation.y - SIGHT_RANGE,
      cog_transform.translation.x + HALF_COG_SIZE + SIGHT_RANGE,
      cog_transform.translation.y + COG_SIZE + SIGHT_RANGE,
    );

    for (toon_entity, toon_transform) in toons_query {
      let toon_rect = Rect::new(
        toon_transform.translation.x - HALF_TOON_SIZE,
        toon_transform.translation.y,
        toon_transform.translation.x + HALF_TOON_SIZE,
        toon_transform.translation.y + TOON_SIZE,
      );
      if !cog_sight_rect.intersect(toon_rect).is_empty() {
        commands.entity(cog_entity).insert(CombatTarget(toon_entity));
        continue;
      }
    }
  }
}

fn update_cogs_follow_target(
  mut commands: Commands,
  cogs_query: Query<
    (Entity, &mut Transform, &mut CogAnimation, &CogRegionMember, &mut UnitFacing, &UnitMovement, &CombatTarget),
    (With<Cog>, With<CombatHealth>, Without<Toon>)
  >,
  cog_regions_query: Query<&CogRegion>,
  toons_query: Query<&Transform, (With<Toon>, With<CombatHealth>, Without<Cog>)>,
  time: Res<Time>,
) {
  let delta_secs = time.delta_secs();
  for (
    cog_entity,
    mut cog_transform,
    mut cog_animation,
    cog_region_member,
    mut cog_facing,
    cog_movement,
    cog_target,
  ) in cogs_query {
    // Respect movement flag
    if !cog_movement.is_movement_enabled {
      continue;
    }

    let distance = cog_movement.speed * delta_secs * 100.;

    let cog_start_x = cog_transform.translation.x;
    let cog_start_y = cog_transform.translation.y;
    let cog_start_z = cog_transform.translation.z;
    
    let cog_sight_rect = Rect::new(
      cog_transform.translation.x - HALF_COG_SIZE - SIGHT_RANGE,
      cog_transform.translation.y - SIGHT_RANGE,
      cog_transform.translation.x + HALF_COG_SIZE + SIGHT_RANGE,
      cog_transform.translation.y + COG_SIZE + SIGHT_RANGE,
    );
    let cog_rect = Rect::new(
      cog_transform.translation.x - TILE_SIZE,
      cog_transform.translation.y,
      cog_transform.translation.x + TILE_SIZE,
      cog_transform.translation.y + TILE_SIZE,
    );

    if let Ok(toon_transform) = toons_query.get(**cog_target) {
      let toon_rect = Rect::new(
        toon_transform.translation.x - HALF_TOON_SIZE,
        toon_transform.translation.y,
        toon_transform.translation.x + HALF_TOON_SIZE,
        toon_transform.translation.y + TOON_SIZE,
      );

      // Can no longer see target
      if cog_sight_rect.intersect(toon_rect).is_empty() {
        // Stop walking
        if **cog_animation == AssetCogAnimationCode::Walk {
          *cog_animation = CogAnimation(AssetCogAnimationCode::Idle);
        }
        // Abandon target
        commands.entity(cog_entity).remove::<CombatTarget>();
        continue;
      }

      // Already at toon
      if !cog_rect.intersect(toon_rect).is_empty() {
        // Stop walking
        if **cog_animation == AssetCogAnimationCode::Walk {
          *cog_animation = CogAnimation(AssetCogAnimationCode::Idle);
        }
        continue;
      }

      // Move/face cog to toon x
      let mut did_move_x = false;
      if cog_rect.max.x < toon_rect.min.x {
        // Left to right
        cog_transform.translation.x += distance;
        did_move_x = true;
        
        if **cog_facing != FacingCode::Right {
          *cog_facing = UnitFacing(FacingCode::Right);
        }
      } else if cog_rect.min.x > toon_rect.max.x {
        // Right to left
        cog_transform.translation.x -= distance;
        did_move_x = true;

        if **cog_facing != FacingCode::Left {
          *cog_facing = UnitFacing(FacingCode::Left);
        }
      }

      // Move/face cog to toon y
      let mut did_move_y = false;
      if cog_rect.max.y < toon_rect.min.y {
        cog_transform.translation.y += distance;
        did_move_y = true;
      } else if cog_rect.min.y > toon_rect.max.y {
        cog_transform.translation.y -= distance;
        did_move_y = true;
      }

      // Did move
      if did_move_x || did_move_y {
        // Update animation
        if **cog_animation != AssetCogAnimationCode::Walk {
          *cog_animation = CogAnimation(AssetCogAnimationCode::Walk);
        }
      }

      // Update z
      if did_move_y {
        cog_transform.translation.z = get_z_from_y(cog_transform.translation.y);
      }

      // Do not leave region
      if let Ok(cog_region) = cog_regions_query.get(**cog_region_member) {
        // If cog left region, go back and drop target
        if !cog_region.bounds.contains(cog_transform.translation.truncate()) {
          cog_transform.translation.x = cog_start_x;
          cog_transform.translation.y = cog_start_y;
          cog_transform.translation.z = cog_start_z;

          if **cog_animation == AssetCogAnimationCode::Walk {
            *cog_animation = CogAnimation(AssetCogAnimationCode::Idle);
          }

          commands.entity(cog_entity).remove::<CombatTarget>();
          continue;
        }

        // If toon left region, drop target
        if cog_region.bounds.intersect(toon_rect).is_empty() {
          commands.entity(cog_entity).remove::<CombatTarget>();
        }
      }

      // Do not collide with other cogs in the region
    }
  }
}

// TODO move cogs to player if in range
// TODO cogs face place if in range

fn poll_map_load_requested(
  mut commands: Commands,
  mut message_reader: MessageReader<LoadMapRequested>,
  cogs_query: Query<Entity, With<Cog>>,
  cog_regions_query: Query<Entity, With<CogRegion>>,
) {
  if message_reader.is_empty() {
    return;
  }
  message_reader.clear();

  for cog_entity in cogs_query {
    commands.entity(cog_entity).despawn();
  }

  for cog_region_entity in cog_regions_query {
    commands.entity(cog_region_entity).despawn();
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