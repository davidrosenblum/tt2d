use bevy::app::{Plugin, Update};
use bevy::camera::Camera2d;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::query::{Changed, With, Without};
use bevy::ecs::system::{Commands, If, Query, Res};
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::log::{info, warn};
use bevy::math::{Rect, StableInterpolate, Vec2};
use bevy::sprite::Sprite;
use bevy::time::Time;
use bevy::transform::components::Transform;

use crate::assets::tiled_map_json::{TiledMapJsonObjectPropertyName, TiledMapJsonObjectType};
use crate::components::player_controlled::PlayerControlled;
use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::teleport_region::TeleportRegion;
use crate::components::toon::Toon;
use crate::components::toon_animation::ToonAnimation;
use crate::components::toon_companion_active::ToonCompanionActive;
use crate::components::toon_sprite::ToonSprite;
use crate::components::unit_facing::UnitFacing;
use crate::components::unit_movement::UnitMovement;
use crate::constants::{TILE_SIZE, TOON_SIZE, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use crate::messages::load_map_requested::LoadMapRequested;
use crate::messages::loaded_map::{LoadedMap};
use crate::models::asset_toon_animation_code::AssetToonAnimationCode;
use crate::models::asset_toon_sprite_code::AssetToonSpriteCode;
use crate::models::facing_code::FacingCode;
use crate::models::toon_companion_code::ToonCompanionCode;
use crate::models::toon_npc_code::ToonNpcCode;
use crate::plugins::toon::toon_utils::{build_toon_bundle, build_toon_companion_bundle, build_toon_npc_bundle};
use crate::resources::map_context::MapContext;
use crate::resources::toon_sprite_store::ToonSpriteStore;
use crate::utils::get_z_from_y::get_z_from_y;
use crate::utils::normalize_tiled_point::normalize_tiled_point;

const HALF_TOON_SIZE: f32 = TOON_SIZE / 2.;

pub struct ToonPlugin;

impl Plugin for ToonPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(Update, (
      update_player_keyboard_movement,
      update_camera_to_player,
      update_toon_facing_direction, 
      update_toon_companion_movement,
      update_toon_animation_frame,
      check_teleport_region_collision,
    ));
    app.add_systems(Update, (poll_map_load_requested, poll_map_loaded));
  }
}

fn update_toon_animation_frame(
  toons_query: Query<(&ToonSprite, &ToonAnimation, &mut SpriteAnimationState, &mut Sprite), With<Toon>>,
  time: Res<Time>,
  toon_sprite_store: If<Res<ToonSpriteStore>>,
) {
  let delta = time.delta();
  for (toon_sprite, toon_animation, mut spritesheet_animation_state, mut sprite) in toons_query {
    spritesheet_animation_state.timer.tick(delta);

    if !spritesheet_animation_state.timer.just_finished() {
      continue;
    }

    let Some(frames) = toon_sprite_store.sprite_map.get_frames(toon_sprite, toon_animation) else {
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

fn update_toon_facing_direction(
  toons_query: Query<(&UnitFacing, &mut Transform), (With<Toon>, Changed<UnitFacing>)>,
) {
  for (unit_facing, mut transform) in toons_query {
    transform.scale.x = match **unit_facing {
      FacingCode::Left => -1.,
      FacingCode::Right => 1.,
    };
  }
}

fn update_toon_companion_movement(
  player_query: Query<&mut Transform, With<PlayerControlled>>,
  toons_companions_query: Query<
    (&UnitMovement, &mut UnitFacing, &mut ToonAnimation,&mut Transform),
    (With<ToonCompanionActive>, Without<PlayerControlled>),
  >,
  time: Res<Time>,
) {
  let Ok(player_transform) = player_query.single() else {
    return;
  };
  let player_follow_box = Rect::new(
    player_transform.translation.x - HALF_TOON_SIZE,
    player_transform.translation.y,
    player_transform.translation.x + HALF_TOON_SIZE,
    player_transform.translation.y + TOON_SIZE,
  );

  let delta_secs = time.delta_secs();

  for (
    unit_movement,
    mut unit_facing,
    mut toon_animation,
    mut transform,
  ) in toons_companions_query {
    // Respect movement flag
    if !unit_movement.is_movement_enabled {
      continue;
    }
    let distance = unit_movement.speed * delta_secs * 100.;

    let mut did_move_x = false;
    if transform.translation.x > player_follow_box.max.x {
      // Move right to left
      transform.translation.x -= distance;
      did_move_x = true;

      if **unit_facing != FacingCode::Left {
        *unit_facing = UnitFacing(FacingCode::Left)
      }
    } else if transform.translation.x < player_follow_box.min.x  {
      // Move left to right
       transform.translation.x += distance;
       did_move_x = true;  

      if **unit_facing != FacingCode::Right {
        *unit_facing = UnitFacing(FacingCode::Right);
      }
    }

    let mut did_move_y = false;
    if transform.translation.y > player_follow_box.max.y {
      // Move bottom to top
      transform.translation.y -= distance;
      did_move_y = true;
    } else if transform.translation.y < player_follow_box.min.y  {
      // Move top to bottom
      transform.translation.y += distance;
      did_move_y = true;  
    }

    if did_move_y {
      transform.translation.z = get_z_from_y(transform.translation.y);
    }

    if did_move_x || did_move_y {
      if **toon_animation != AssetToonAnimationCode::Walk {
        *toon_animation = ToonAnimation(AssetToonAnimationCode::Walk);
      }
    }
  }
}

fn update_camera_to_player(
  mut camera_query: Query<&mut Transform, With<Camera2d>>,
  player_query: Query<&Transform, (With<PlayerControlled>, Without<Camera2d>)>,
  map_context: If<Res<MapContext>>,
  time: Res<Time>,
) {
  let Ok(mut camera_transform) = camera_query.single_mut() else {
    return;
  };

  let Ok(player_transform) = player_query.single() else {
    return;
  };

  let target = &player_transform.translation;
  const DECAY_RATE: f32 = 2.;
  let delta_secs = time.delta_secs();
  camera_transform.translation.smooth_nudge(target, DECAY_RATE, delta_secs);

  const HALF_VIEWPORT_WIDTH: f32 = VIEWPORT_WIDTH as f32 / 2.;
  camera_transform.translation.x = camera_transform.translation.x.clamp(
    map_context.bounds.min.x + HALF_VIEWPORT_WIDTH,
    map_context.bounds.max.x - HALF_VIEWPORT_WIDTH,
  );

  const HALF_VIEWPORT_HEIGHT: f32 = VIEWPORT_HEIGHT as f32 / 2.;
  camera_transform.translation.y = camera_transform.translation.y.clamp(
    map_context.bounds.min.y + HALF_VIEWPORT_HEIGHT,
    map_context.bounds.max.y - HALF_VIEWPORT_HEIGHT,
  );
}

fn update_player_keyboard_movement(
  mut player_query: Query<(&UnitMovement, &mut UnitFacing, &mut ToonAnimation, &mut Transform), With<PlayerControlled>>,
  map_context: If<Res<MapContext>>,
  button_input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
  let Ok((unit_movement, mut unit_facing, mut toon_animation, mut transform)) = player_query.single_mut() else {
    return;
  };

  if !unit_movement.is_movement_enabled {
    return;
  }

  let delta_secs = time.delta_secs();
  let distance = unit_movement.speed * delta_secs * 100. * 3.; // TODO remove 3.

  // Move up or down
  let mut did_move_y = false;
  const KEYS_UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
  const KEYS_DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
  if button_input.any_pressed(KEYS_UP) {
    transform.translation.y += distance;
    did_move_y = true;
  } else if button_input.any_pressed(KEYS_DOWN) {
    transform.translation.y -= distance;
    did_move_y = true;
  }

  // Move left or right
  let mut did_move_x = false;
  const KEYS_LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
  const KEYS_RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
  if button_input.any_pressed(KEYS_LEFT) {
    transform.translation.x -= distance;
    did_move_x = true;
    if **unit_facing != FacingCode::Left {
      *unit_facing = UnitFacing(FacingCode::Left)
    }
  } else if button_input.any_pressed(KEYS_RIGHT) {
    transform.translation.x += distance;
    did_move_x = true;
    if **unit_facing != FacingCode::Right {
      *unit_facing = UnitFacing(FacingCode::Right);
    }
  }

  // Enforce map bounds
   if did_move_x {
    transform.translation.x = transform.translation.x.clamp(
      // Bottom center (x=center)
      map_context.bounds.min.x + HALF_TOON_SIZE,
      map_context.bounds.max.x - HALF_TOON_SIZE,
    );
  }
  if did_move_y {
    transform.translation.y = transform.translation.y.clamp(
      // Bottom center (y=bottom)
      map_context.bounds.min.y,
      map_context.bounds.max.y - TOON_SIZE,
    );
    transform.translation.z = get_z_from_y(transform.translation.y);
  }

  // Update animation from anything to walk
  if
    button_input.any_just_pressed(KEYS_UP) || 
    button_input.any_just_pressed(KEYS_DOWN) ||
    button_input.any_just_pressed(KEYS_LEFT) || 
    button_input.any_just_pressed(KEYS_RIGHT)
  {
    if **toon_animation != AssetToonAnimationCode::Walk {
      *toon_animation = ToonAnimation(AssetToonAnimationCode::Walk);
    }
  }

  // Update animation from anything to idle
  if
    button_input.any_just_released(KEYS_UP) || 
    button_input.any_just_released(KEYS_DOWN) ||
    button_input.any_just_released(KEYS_LEFT) || 
    button_input.any_just_released(KEYS_RIGHT)
  {
    if **toon_animation != AssetToonAnimationCode::Idle {
      *toon_animation = ToonAnimation(AssetToonAnimationCode::Idle);
    }
  }

  // TODO check collision
  if unit_movement.is_collision_enabled {}
}

fn check_teleport_region_collision(
  player_query: Query<&Transform, (With<PlayerControlled>, Changed<Transform>)>,
  teleport_regions_query: Query<&TeleportRegion>,
  mut load_map_requested_writer: MessageWriter<LoadMapRequested>,
) {
  let Ok(player_transform) = player_query.single() else {
    return;
  };

  let player_rect = Rect::new(
    player_transform.translation.x,
    player_transform.translation.y,
    player_transform.translation.x + TILE_SIZE,
    player_transform.translation.y + TILE_SIZE,
  );
  for teleport_region in teleport_regions_query {
    if !teleport_region.bounds.intersect(player_rect).is_empty() {
      info!("Player colliding with teleport region for {:?}", teleport_region.map_code);
      load_map_requested_writer.write(LoadMapRequested { map_code: teleport_region.map_code });
      return;
    }
  }
}

fn poll_map_load_requested(
  mut commands: Commands,
  mut message_reader: MessageReader<LoadMapRequested>,
  toons_query: Query<Entity, With<Toon>>,
) {
  if message_reader.is_empty() {
    return;
  }
  message_reader.clear();

  for toon_entity in toons_query {
    commands.entity(toon_entity).despawn();
  }
}

fn poll_map_loaded(
  mut commands: Commands,
  mut message_reader: MessageReader<LoadedMap>,
  map_context: If<Res<MapContext>>,
  toon_sprite_store: If<Res<ToonSpriteStore>>,
) {
  let Some(message) = message_reader.read().last() else {
    return;
  };

  // Spawn toons
  for map_object in &message.map_objects {
    match map_object.object_type {
      TiledMapJsonObjectType::PlayerSpawner => {
        let position_map = Vec2::new(map_object.x as f32, map_object.y as f32);
        let position = normalize_tiled_point(&position_map, message.tilewidth, message.tileheight, message.height);

        if let Some(toon_bundle) = build_toon_bundle(
          AssetToonSpriteCode::Player,
          FacingCode::Right,
          "Player",
          position,
          &toon_sprite_store
        ) {
          commands.spawn((
            PlayerControlled,
            toon_bundle,
          ));
          info!("Spawned toon player");
        } else {
          warn!("PlayerSpawner failed to spawn player");
        }
      },
      TiledMapJsonObjectType::ToonCompanionSpawner => {
        let Some(properties) = &map_object.properties else {
          warn!("ToonCompanionSpawner missing properties");
          continue;
        };

        let Some(companion_code) = properties.iter().find_map(|prop| {
          if prop.name == TiledMapJsonObjectPropertyName::ToonCompanionSpawnerCompanion {
            return prop.value.parse::<ToonCompanionCode>().ok();
          }
          None
        }) else {
          warn!("ToonCompanionSpawner missing or invalid companion property");
          continue;
        };

        let facing_code = properties.iter().find_map(|prop| {
          if prop.name == TiledMapJsonObjectPropertyName::ToonCompanionSpawnerCompanionFacing {
            return prop.value.parse::<FacingCode>().ok();
          }
          None
        }).unwrap_or_default();

        let position_map = Vec2::new(map_object.x as f32, map_object.y as f32);
        let position = normalize_tiled_point(&position_map, message.tilewidth, message.tileheight, message.height);

        if let Some(toon_bundle) = build_toon_companion_bundle(
          &companion_code,
          &facing_code,
          position,
          &toon_sprite_store,
        ) {
          let toon_entity = commands.spawn(toon_bundle).id();
          info!("Spawned toon companion: {:?}", companion_code);

          // Only "activate" companions if the map allows it
          if map_context.map_data.companions_enabled {
            commands.entity(toon_entity).insert(ToonCompanionActive);
            info!("Making companion active: {:?}", companion_code);
          }
        } else {
          warn!("ToonCompanionSpawner companion data missing");
        }
      },
      TiledMapJsonObjectType::ToonNpcSpawner => {
        let Some(properties) = &map_object.properties else {
          warn!("ToonNpcSpawner missing properties");
          continue;
        };

        let Some(npc_code) = properties.iter().find_map(|prop| {
          if prop.name == TiledMapJsonObjectPropertyName::ToonNpcSpawnerNpc {
            return prop.value.parse::<ToonNpcCode>().ok();
          }
          None
        }) else {
          warn!("ToonNpcSpawner missing or invalid npc property");
          continue;
        };

        let facing_code = properties.iter().find_map(|prop| {
          if prop.name == TiledMapJsonObjectPropertyName::ToonNpcSpawnerFacing {
            return prop.value.parse::<FacingCode>().ok();
          }
          None
        }).unwrap_or_default();

        let position_map = Vec2::new(map_object.x as f32, map_object.y as f32);
        let position = normalize_tiled_point(&position_map, message.tilewidth, message.tileheight, message.height);

        if let Some(toon_bundle) = build_toon_npc_bundle(
          &npc_code,
          &facing_code,
          position,
          &toon_sprite_store,
        ) {
          commands.spawn(toon_bundle);
          info!("Spawned toon npc: {:?}", npc_code);
        } else {
          warn!("ToonNpcSpawner npc data missing: {:?}", npc_code);
        }
      },
      _ => (),
    }
  }
}