use bevy::app::{Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageReader;
use bevy::ecs::query::{Changed, With, Without};
use bevy::ecs::system::{Commands, If, Query, Res};
use bevy::log::warn;
use bevy::math::Rect;
use bevy::sprite::Sprite;
use bevy::time::Time;
use bevy::transform::components::Transform;

use crate::components::cog::Cog;
use crate::components::cog_animation::CogAnimation;
use crate::components::cog_behavior::CogBehavior;
use crate::components::cog_region::CogRegion;
use crate::components::cog_region_member::CogRegionMember;
use crate::components::cog_spawn_point::CogSpawnPoint;
use crate::components::cog_sprite::CogSprite;
use crate::components::combat_health::CombatHealth;
use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::toon::Toon;
use crate::components::unit_facing::UnitFacing;
use crate::components::unit_movement::UnitMovement;
use crate::constants::TILE_SIZE;
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
    // app.add_systems(Update, (update_cogs_acquire_target, update_cogs_follow_target));
    app.add_systems(Update, update_cog_behavior);
    app.add_systems(Update, (poll_map_load_requested, poll_map_loaded));
  }
}

const SIGHT_RANGE: f32 = TILE_SIZE * 2.;
const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.;

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

fn update_cog_behavior(
  // mut commands: Commands,
  cogs_query: Query<
    (&mut Transform, &mut CogAnimation, &mut CogBehavior, Option<&CogRegionMember>, &CogSpawnPoint, &mut UnitFacing, &UnitMovement),
    (With<Cog>, With<CombatHealth>, Without<Toon>)
  >,
  cog_regions_query: Query<&CogRegion>,
  toons_query: Query<(Entity, &Transform), (With<Toon>, With<CombatHealth>, Without<Cog>)>,
  time: Res<Time>,
) {
  let delta_secs = time.delta_secs();
  for (
    mut cog_transform,
    mut cog_animation,
    mut cog_behavior,
    cog_region_member_opt,
    cog_spawn_point,
    mut cog_facing,
    cog_movement,
  ) in cogs_query {    
    match *cog_behavior {
      CogBehavior::LookForTarget => {
        let cog_sight_rect = Rect::new(
          cog_transform.translation.x - HALF_TILE_SIZE - SIGHT_RANGE,
          cog_transform.translation.y - SIGHT_RANGE,
          cog_transform.translation.x + HALF_TILE_SIZE + SIGHT_RANGE,
          cog_transform.translation.y + TILE_SIZE + SIGHT_RANGE,
        );

        for (toon_entity, toon_transform) in toons_query {
          let toon_tile_rect = Rect::new(
            toon_transform.translation.x - HALF_TILE_SIZE,
            toon_transform.translation.y,
            toon_transform.translation.x + HALF_TILE_SIZE,
            toon_transform.translation.y + TILE_SIZE,
          );
          if !toon_tile_rect.intersect(cog_sight_rect).is_empty() {
            if let Some(cog_region_member) = cog_region_member_opt {
              if let Ok(cog_region) = cog_regions_query.get(**cog_region_member) {
                if !cog_region.bounds.intersect(toon_tile_rect).is_empty() {
                  // Toon must be in bounds AND in sight (already checked)
                  *cog_behavior = CogBehavior::Targeting(toon_entity);
                }
              }
            } else {
              // Do not need to check toon is in region because there is no linked cog region (standalone cog)
              *cog_behavior = CogBehavior::Targeting(toon_entity);
            }
          }
        }
      },
      CogBehavior::ReturnToSpawn => {
        if !cog_movement.is_movement_enabled {
          continue;
        }
        let distance = cog_movement.speed * delta_secs * 100.;

        // Handle x-axis movement, trunc prevents jitter
        let mut did_move_x = false;
        if cog_transform.translation.x.trunc() > cog_spawn_point.x.trunc() {
          // Right to left
          cog_transform.translation.x -= distance;
          did_move_x = true;

          if **cog_facing != FacingCode::Left {
            *cog_facing = UnitFacing(FacingCode::Left);
          }
        } else if cog_transform.translation.x.trunc() < cog_spawn_point.x.trunc() {
          // Left to right
          cog_transform.translation.x += distance;
          did_move_x = true;

          if **cog_facing != FacingCode::Right {
            *cog_facing = UnitFacing(FacingCode::Right);
          }
        }

        // Handle y-axis movement
        let mut did_move_y = false;
        if cog_transform.translation.y > cog_spawn_point.y {
          cog_transform.translation.y -= distance;
          did_move_y = true;
        } else if cog_transform.translation.y < cog_spawn_point.y {
          cog_transform.translation.y += distance;
          did_move_y = true;
        }

        // Update z (depth sort)
        if did_move_y {
          cog_transform.translation.z = get_z_from_y(cog_transform.translation.y);
        }

        // Update animation if moving
        if did_move_x || did_move_y {
          if **cog_animation != AssetCogAnimationCode::Walk {
            *cog_animation = CogAnimation(AssetCogAnimationCode::Walk);
          }
        }

        // Reached initial spawn point
        if cog_transform.translation.x.trunc() == cog_spawn_point.x.trunc() {
          if cog_transform.translation.y.trunc() == cog_spawn_point.y.trunc() {
            *cog_behavior = CogBehavior::LookForTarget;
          }
        }
      },
      CogBehavior::Targeting(target_entity) => {
        let Ok((_, toon_transform)) = toons_query.get(target_entity) else {
          // Referenced toon entity does not exist
          warn!("Toon target missing");
          *cog_behavior = CogBehavior::ReturnToSpawn;
          continue;
        };

         let cog_tile_rect = Rect::new(
          cog_transform.translation.x - HALF_TILE_SIZE,
          cog_transform.translation.y,
          cog_transform.translation.x + HALF_TILE_SIZE,
          cog_transform.translation.y + TILE_SIZE,
        );

        // If the cog leaves the region, abandon the target and return to spawn
        if let Some(cog_region_member) = cog_region_member_opt {
          if let Ok(cog_region) = cog_regions_query.get(**cog_region_member) {
            if cog_region.bounds.intersect(cog_tile_rect).is_empty() {
              *cog_behavior = CogBehavior::ReturnToSpawn;
              continue;
            }
          }
        }

        let toon_tile_rect = Rect::new(
          toon_transform.translation.x - HALF_TILE_SIZE,
          toon_transform.translation.y,
          toon_transform.translation.x + HALF_TILE_SIZE,
          toon_transform.translation.y + TILE_SIZE,
        );

        // If the toon is too far away, then follow them, otherwise attack them if they are close enough
        if cog_tile_rect.intersect(toon_tile_rect).is_empty() {
          // Toon is out of range, cog must walk to toon
          if !cog_movement.is_movement_enabled {
            continue;
          }
          let distance = cog_movement.speed * delta_secs * 100.;

          let mut did_move_x = false;
          if cog_tile_rect.min.x > toon_tile_rect.max.x {
            // Right to left
            cog_transform.translation.x -= distance;
            did_move_x = true;

            if **cog_facing != FacingCode::Left {
              *cog_facing = UnitFacing(FacingCode::Left);
            }
          } else if cog_tile_rect.max.x < toon_tile_rect.min.x {
            // Left to right
            cog_transform.translation.x += distance;
            did_move_x = true;

            if **cog_facing != FacingCode::Right {
              *cog_facing = UnitFacing(FacingCode::Right);
            }
          }

          let mut did_move_y = false;
          if cog_tile_rect.min.y > toon_tile_rect.max.y {
            cog_transform.translation.y -= distance;
            did_move_y = true;
          } else if cog_tile_rect.max.y < toon_tile_rect.min.y  {
            cog_transform.translation.y += distance;
            did_move_y = true;
          }

          if did_move_y {
            cog_transform.translation.z = get_z_from_y(cog_transform.translation.y);
          }

          if did_move_x || did_move_y {
            if **cog_animation != AssetCogAnimationCode::Walk {
              *cog_animation = CogAnimation(AssetCogAnimationCode::Walk);
            }
          }
        } else {
          // Toon is in range, cog is safe to attack
          if **cog_animation == AssetCogAnimationCode::Walk {
            *cog_animation = CogAnimation(AssetCogAnimationCode::Idle);
          }
        }
      },
    }
  }
}

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