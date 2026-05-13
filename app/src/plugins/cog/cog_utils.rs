use bevy::ecs::system::Commands;
use bevy::image::TextureAtlas;
use bevy::log::{info, warn};
use bevy::math::{Rect, Vec2};
use bevy::sprite::{Anchor, Sprite};
use bevy::transform::components::Transform;
use rand::RngExt;
use uuid::Uuid;

use crate::assets::tiled_map_json::{TiledMapJsonObject, TiledMapJsonObjectPropertyName, TiledMapJsonObjectType};
use crate::components::cog::Cog;
use crate::components::cog_animation::CogAnimation;
use crate::components::cog_behavior::CogBehavior;
use crate::components::cog_bundle::CogBundle;
use crate::components::cog_region::CogRegion;
use crate::components::cog_region_member::CogRegionMember;
use crate::components::cog_spawn_point::CogSpawnPoint;
use crate::components::cog_sprite::CogSprite;
use crate::components::combat_bundle::CombatBundle;
use crate::components::combat_health::CombatHealth;
use crate::components::combat_melee_attack::CombatMeleeAttack;
use crate::components::sprite_animation_state::SpriteAnimationState;
use crate::components::unit::Unit;
use crate::components::unit_facing::UnitFacing;
use crate::components::unit_movement::UnitMovement;
use crate::constants::{COG_SIZE, TILE_SIZE};
use crate::data::cog_data::{COG_DATA_STORE, CogData};
use crate::models::asset_cog_animation_code::AssetCogAnimationCode;
use crate::models::cog_department_code::CogDepartmentCode;
use crate::models::facing_code::FacingCode;
use crate::resources::cog_sprite_store::CogSpriteStore;
use crate::utils::get_z_from_y::get_z_from_y;
use crate::utils::normalize_tiled_point::normalize_tiled_point;

pub fn build_cog_bundle(
  cog_data: &CogData,
  facing_code: FacingCode,
  position: Vec2,
  cog_sprite_store: &CogSpriteStore,
) -> Option<CogBundle> {
  let animation_code = AssetCogAnimationCode::Idle;

  let Some(frame) = cog_sprite_store.sprite_map.get_first_frame(&cog_data.sprite_code, &animation_code) else {
    return None;
  };

  let mut sprite = Sprite::from_atlas_image(
    cog_sprite_store.image_handle.clone(),
    TextureAtlas {
      index: frame.layout_index,
      layout: cog_sprite_store.atlas_layout_handle.clone(),
    },
  );
  sprite.custom_size = Some(Vec2::splat(COG_SIZE));

  let cog_bundle = CogBundle {
    anchor: Anchor::BOTTOM_CENTER,
    cog: Cog {
      name: cog_data.name.to_string(),
      department_code: cog_data.department_code,
    },
    cog_animation: CogAnimation(animation_code),
    cog_behavior: CogBehavior::LookForTarget,
    cog_spawn_point: CogSpawnPoint(position),
    cog_sprite: CogSprite(cog_data.sprite_code),
    sprite,
    sprite_animation_state: SpriteAnimationState::new(frame.duration),
    transform: Transform::from_xyz(position.x, position.y, get_z_from_y(position.y)),
    unit: Unit {
      id: Uuid::new_v4(),
    },
    unit_facing: UnitFacing(facing_code),
    unit_movement: UnitMovement::new(1.),
  };
  Some(cog_bundle)
}

pub fn build_cog_bundles_from_dept_tier(
  cog_department_code: CogDepartmentCode,
  facing_code: FacingCode,
  tier: u32,
  position: Vec2,
  cog_sprite_store: &CogSpriteStore,
) -> Option<(CogBundle, CombatBundle)> {
  let Some(cog_data) = COG_DATA_STORE.iter().find(|cog_data| {
    cog_data.department_code == cog_department_code && cog_data.tier == tier
  }) else {
    return None;
  };
  let cog_bundle = build_cog_bundle(cog_data, facing_code, position, cog_sprite_store)?;
  let combat_bundle = CombatBundle {
    health: CombatHealth::new(cog_data.health as f32),
    melee_attack: CombatMeleeAttack::new(cog_data.melee_attack.cooldown, cog_data.melee_attack.damage_range),
  };
  Some((cog_bundle, combat_bundle))
}

pub fn process_map_cog_spawner(
  commands: &mut Commands,
  map_object: &TiledMapJsonObject,
  cog_sprite_store: &CogSpriteStore,
  tilewidth: u32,
  tileheight: u32,
  map_height: u32,
) {
  let Some(properties) = &map_object.properties else {
    warn!("CogSpawner missing properties");
    return;
  };

  let Some(cog_department_code) = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::CogSpawnerDepartment {
      return prop.value.parse::<CogDepartmentCode>().ok();
    }
    None
  }) else {
    warn!("CogSpawner missing or invalid department");
    return;
  };

  let Some(tier) = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::CogSpawnerTier {
      return prop.value.parse::<u32>().ok();
    }
    None
  }) else {
    warn!("CogSpawner missing or invalid tier");
    return;
  };

  let facing_code = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::CogSpawnerFacing {
      return prop.value.parse::<FacingCode>().ok();
    }
    None
  }).unwrap_or_default();

  let position_map = Vec2::new(map_object.x as f32, map_object.y as f32);
  let position = normalize_tiled_point(&position_map, tilewidth, tileheight, map_height);
  
  if let Some(bundles) = build_cog_bundles_from_dept_tier(
    cog_department_code,
    facing_code,
    tier,
    position,
    &cog_sprite_store,
  ) {
    commands.spawn(bundles);
    info!("Spawned cog: {:?}:{:?}", cog_department_code, tier);
  } else {
    warn!("CogSpawner cog data missing: {:?}:{:?}", cog_department_code, tier);
  }
}

pub fn process_map_cog_region(
  commands: &mut Commands,
  map_object: &TiledMapJsonObject,
  cog_sprite_store: &CogSpriteStore,
  tilewidth: u32,
  tileheight: u32,
  map_height: u32,
) {
  let Some(properties) = &map_object.properties else {
    warn!("CogRegion missing properties");
    return;
  };

  let Some(count) = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::CogRegionCount {
      return prop.value.parse::<u32>().ok();
    }
    None
  }) else {
    warn!("CogRegion missing or invalid count");
    return;
  };

  let Some(cog_department_code) = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::CogRegionDepartment {
      return prop.value.parse::<CogDepartmentCode>().ok();
    }
    None
  }) else {
    warn!("CogRegion missing or invalid cog department");
    return;
  };

  let Some(difficulty) = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::CogRegionDifficulty {
      return prop.value.parse::<u32>().ok();
    }
    None
  }) else {
    warn!("CogRegion missing or invalid difficulty");
    return;
  };

  let Some(tier_min) = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::CogRegionTierMin {
      return prop.value.parse::<u32>().ok();
    }
    None
  }) else {
    warn!("CogRegion missing or invalid tier min");
    return;
  };

  let Some(tier_max) = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::CogRegionTierMax {
      return prop.value.parse::<u32>().ok();
    }
    None
  }) else {
    warn!("CogRegion missing or invalid tier max");
    return;
  };

  let count_safe = std::cmp::max(1, count);
  let _difficult_safe = std::cmp::max(1, difficulty);
  let tier_min_safe = std::cmp::max(1, tier_min);
  let tier_max_safe = std::cmp::max(tier_min_safe, tier_max);

  // Region area
  let scale_x = TILE_SIZE / tilewidth as f32;
  let scale_y = TILE_SIZE / tileheight as f32;
  let region_tiled_position = Vec2::new(map_object.x as f32, map_object.y as f32);
  let region_position = normalize_tiled_point(&region_tiled_position, tilewidth, tileheight, map_height);
  let region_rect = Rect::new(
    region_position.x,
    region_position.y,
    region_position.x + map_object.width as f32 * scale_x,
    region_position.y - map_object.height as f32 * scale_y,
  );

  // Create the region entity
  let cog_region_entity = commands.spawn_empty().id();

  // Randomness
  let mut rng = rand::rng();

  // Populate array with tiers to create based on count and difficulty
  // let tier_range = tier_max_safe - tier_min_safe;
  let mut tiers = Vec::<u32>::new();
  for _ in 0 .. count_safe {
    // TODO apply difficulty
    let random_tier = rng.random_range(tier_min_safe ..= tier_max_safe);
    tiers.push(random_tier);
  }

  // Spawn the cogs
  for tier in tiers {
    let x = rng.random_range(region_rect.min.x ..= region_rect.max.x);
    let y = rng.random_range(region_rect.min.y ..= region_rect.max.y);
    let position = Vec2::new(x, y);

    let facing_code = if x <= region_rect.center().x { FacingCode::Right } else { FacingCode::Left };

    if let Some((cog_bundle, combat_bundle)) = build_cog_bundles_from_dept_tier(
      cog_department_code,
      facing_code,
      tier,
      position,
      &cog_sprite_store,
    ) {
      commands.spawn((
        cog_bundle,
        combat_bundle,
        CogRegionMember(cog_region_entity),
      ));
      info!("Spawned region cog: {:?}:{:?}", cog_department_code, tier);
    } else {
      warn!("CogRegion cog data missing: {:?}:{:?}", cog_department_code, tier);
    }
  }

  // Apply the bounds to the region
  let cog_region = CogRegion {
    bounds: region_rect,
  };
  commands.entity(cog_region_entity).insert(cog_region);
}

pub fn process_map_object(
  commands: &mut Commands,
  map_object: &TiledMapJsonObject,
  cog_sprite_store: &CogSpriteStore,
  tilewidth: u32,
  tileheight: u32,
  map_height: u32,
) {
  match map_object.object_type {
    TiledMapJsonObjectType::CogRegion => {
      process_map_cog_region(commands, map_object, cog_sprite_store, tilewidth, tileheight, map_height);
    },
    TiledMapJsonObjectType::CogSpawner => {
      process_map_cog_spawner(commands, map_object, cog_sprite_store, tilewidth, tileheight, map_height);
    },
    _ => (),
  }
  
}