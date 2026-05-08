use bevy::asset::Handle;
use bevy::ecs::system::Commands;
use bevy::image::Image;
use bevy::log::{info, warn};
use bevy::math::{Rect, Vec2, Vec3};
use bevy::sprite::{Anchor, Sprite};
use bevy::transform::components::Transform;
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType};
use bevy_ecs_tilemap::tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex};

use crate::assets::tiled_map_json::{TiledMapJson, TiledMapJsonLayer, TiledMapJsonObject, TiledMapJsonObjectPropertyName, TiledMapJsonObjectType, TiledMapJsonTileLayer};
use crate::components::structure_tile::StructureTile;
use crate::components::teleport_region::TeleportRegion;
use crate::constants::{STRUCTURE_TOP_Z_INDEX, TERRAIN_Z_INDEX, TILE_SIZE};
use crate::errors::tilemap_spawn_error::TilemapSpawnError;
use crate::models::map_code::MapCode;
use crate::plugins::map::map_constants::{self, TILEMAP_LAYER_INDEX_STRUCTURES_BASE};
use crate::utils::get_z_from_y::get_z_from_y;
use crate::utils::normalize_tiled_point::normalize_tiled_point;

const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.;

pub fn get_terrain_image_path(map_code: &MapCode) -> &'static str {
  match map_code {
    MapCode::TtcPlayground | MapCode::TtcStreet =>
      map_constants::TTC_TILESET_TERRAIN_IMAGE_PATH,

    MapCode::DdPlayground | MapCode::DdStreet =>
      map_constants::DD_TILESET_TERRAIN_IMAGE_PATH,

    MapCode::DgPlayground | MapCode::DgStreet =>
      map_constants::DG_TILESET_TERRAIN_IMAGE_PATH,

    MapCode::MmlPlayground | MapCode::MmlStreet =>
      map_constants::MML_TILESET_TERRAIN_IMAGE_PATH,

    MapCode::BrPlayground | MapCode::BrStreet =>
      map_constants::BR_TILESET_TERRAIN_IMAGE_PATH,
    
    MapCode::DdlPlayground | MapCode::DdlStreet =>
      map_constants::DDL_TILESET_TERRAIN_IMAGE_PATH,
    
    MapCode::CogBuildingBb | MapCode::CogBuildingLb | MapCode::CogBuildingCb | MapCode::CogBuildingSb =>
      map_constants::COGBUILDING_TILESET_TERRAIN_IMAGE_PATH,
    
    MapCode::SbhqCourtyard | MapCode::SbhqFactory | MapCode::SbhqLobby | MapCode::SbhqVp =>
      map_constants::SBHQ_TERRAIN_IMAGE_PATH,
    
    MapCode::CbhqCourtyard | MapCode::CbhqMint | MapCode::CbhqLobby | MapCode::CbhqCfo =>
      map_constants::CBHQ_TERRAIN_IMAGE_PATH,
    
    MapCode::LbhqCourtyard | MapCode::LbhqDaOffice | MapCode::LbhqLobby | MapCode::LbhqCj =>
      map_constants::LBHQ_TERRAIN_IMAGE_PATH,
    
    MapCode::BbhqCourtyard | MapCode::BbhqGolfCourse | MapCode::BbhqLobby | MapCode::BbhqCeo =>
      map_constants::BBHQ_TERRAIN_IMAGE_PATH,
  }
}

pub fn get_structures_image_path(map_code: &MapCode) -> &'static str {
  match map_code {
    MapCode::TtcPlayground | MapCode::TtcStreet =>
      map_constants::TTC_TILESET_STRUCTURES_IMAGE_PATH,

    MapCode::DdPlayground | MapCode::DdStreet =>
      map_constants::DD_TILESET_STRUCTURES_IMAGE_PATH,

    MapCode::DgPlayground | MapCode::DgStreet =>
      map_constants::DG_TILESET_STRUCTURES_IMAGE_PATH,

    MapCode::MmlPlayground | MapCode::MmlStreet =>
      map_constants::MML_TILESET_STRUCTURES_IMAGE_PATH,

    MapCode::BrPlayground | MapCode::BrStreet =>
      map_constants::BR_TILESET_STRUCTURES_IMAGE_PATH,

    MapCode::DdlPlayground | MapCode::DdlStreet =>
      map_constants::DDL_TILESET_STRUCTURES_IMAGE_PATH,

    MapCode::CogBuildingBb | MapCode::CogBuildingLb | MapCode::CogBuildingCb | MapCode::CogBuildingSb =>
      map_constants::COGBUILDING_TILESET_STRUCTURES_IMAGE_PATH,

    MapCode::SbhqCourtyard | MapCode::SbhqFactory | MapCode::SbhqLobby | MapCode::SbhqVp =>
      map_constants::SBHQ_STRUCTURES_IMAGE_PATH,
      
    MapCode::CbhqCourtyard | MapCode::CbhqMint | MapCode::CbhqLobby | MapCode::CbhqCfo =>
    map_constants::CBHQ_STRUCTURES_IMAGE_PATH,

    MapCode::LbhqCourtyard | MapCode::LbhqDaOffice | MapCode::LbhqLobby | MapCode::LbhqCj =>
      map_constants::LBHQ_STRUCTURES_IMAGE_PATH,

    MapCode::BbhqCourtyard | MapCode::BbhqGolfCourse | MapCode::BbhqLobby | MapCode::BbhqCeo =>
      map_constants::BBHQ_STRUCTURES_IMAGE_PATH,
  }
}

pub fn get_tilemap_json_path(map_code: &MapCode) -> &'static str {
  match map_code {
    MapCode::TtcPlayground => map_constants::TTC_PLAYGROUND_TILEMAP_PATH,
    MapCode::TtcStreet => map_constants::TTC_STREETS_TILEMAP_PATH,
    MapCode::DdPlayground => map_constants::DD_PLAYGROUND_TILEMAP_PATH,
    MapCode::DdStreet => map_constants::DD_STREETS_TILEMAP_PATH,
    MapCode::DgPlayground => map_constants::DG_PLAYGROUND_TILEMAP_PATH,
    MapCode::DgStreet => map_constants::DG_STREETS_TILEMAP_PATH,
    MapCode::MmlPlayground =>map_constants:: MML_PLAYGROUND_TILEMAP_PATH,
    MapCode::MmlStreet => map_constants::MML_STREETS_TILEMAP_PATH,
    MapCode::BrPlayground => map_constants::BR_PLAYGROUND_TILEMAP_PATH,
    MapCode::BrStreet => map_constants::BR_STREETS_TILEMAP_PATH,
    MapCode::DdlPlayground => map_constants::DDL_PLAYGROUND_TILEMAP_PATH,
    MapCode::DdlStreet => map_constants::DDL_STREETS_TILEMAP_PATH,
    MapCode::CogBuildingBb => map_constants::COGBUILDING_BB_TILEMAP_PATH,
    MapCode::CogBuildingLb => map_constants::COGBUILDING_LB_TILEMAP_PATH,
    MapCode::CogBuildingCb => map_constants::COGBUILDING_CB_TILEMAP_PATH,
    MapCode::CogBuildingSb => map_constants::COGBUILDING_SB_TILEMAP_PATH,
    MapCode::SbhqCourtyard => map_constants::SBHQ_COURTYARD_TILEMAP_PATH,
    MapCode::SbhqFactory => map_constants::SBHQ_FACTORY_TILEMAP_PATH,
    MapCode::SbhqLobby => map_constants::SBHQ_LOBBY_TILEMAP_PATH,
    MapCode::SbhqVp => map_constants::SBHQ_VP_TILEMAP_PATH,
    MapCode::CbhqCourtyard => map_constants::CBHQ_COURTYARD_TILEMAP_PATH,
    MapCode::CbhqMint => map_constants::CBHQ_MINT_TILEMAP_PATH,
    MapCode::CbhqLobby => map_constants::CBHQ_LOBBY_TILEMAP_PATH,
    MapCode::CbhqCfo => map_constants::CBHQ_CFO_TILEMAP_PATH,
    MapCode::LbhqCourtyard => map_constants::LBHQ_COURTYARD_TILEMAP_PATH,
    MapCode::LbhqDaOffice => map_constants::LBHQ_DAOFFICE_TILEMAP_PATH,
    MapCode::LbhqLobby => map_constants::LBHQ_LOBBY_TILEMAP_PATH,
    MapCode::LbhqCj => map_constants::LBHQ_CJ_TILEMAP_PATH,
    MapCode::BbhqCourtyard => map_constants::BBHQ_COURTYARD_TILEMAP_PATH,
    MapCode::BbhqGolfCourse => map_constants::BBHQ_GOLFCOURSE_TILEMAP_PATH,
    MapCode::BbhqLobby => map_constants::BBHQ_LOBBY_TILEMAP_PATH,
    MapCode::BbhqCeo => map_constants::BBHQ_CEO_TILEMAP_PATH,
  }
}

/** Figure out the area of the map in pixels. */
pub fn get_map_bounds(tilemap_json: &TiledMapJson) -> Rect {
  Rect::new(
    0., // Left
    0., // Bottom
    tilemap_json.width as f32 * TILE_SIZE, // Right
    tilemap_json.height as f32 * TILE_SIZE, // Top
  )
}

pub fn get_collision_mask(tilemap_json: &TiledMapJson) -> Vec<bool> {
  let Some(layer) = tilemap_json.layers.get(TILEMAP_LAYER_INDEX_STRUCTURES_BASE) else {
    warn!("get_collision_mask: Structures base layer is missing");
    return vec![];
  };

  let TiledMapJsonLayer::Tile(tile_layer) = layer else {
    warn!("get_collision_mask: Structures base layer is not a tile layer");
    return vec![];
  };

  tile_layer.data.iter().map(|tile_id| *tile_id > 0).collect()
}

/** Figure out bevy (x, y) in a 1d array. This is not pixels but grid coordinates (think col/row). */
pub fn get_tile_xy(
  index: usize,
  tilemap_width: u32,
  tilemap_height: u32,
) -> (u32, u32) {
  let x = index as u32 % tilemap_width;
  let y = index as u32 / tilemap_width;
  let bevy_y = tilemap_height - 1 - y;
  (x, bevy_y)
}

/** Create a tilemap layer. More performant, but cannot do depth sorting. */
pub fn spawn_tilemap_layer(
  commands: &mut Commands,
  tilemap_json_layer: &TiledMapJsonTileLayer,
  tileset_image_handle: Handle<Image>,
  tilewidth: u32,
  tileheight: u32,
  first_guid: u32,
  z_index: f32,
) {
  // Create the tilemap storage
  let tilemap_size = TilemapSize::new(tilemap_json_layer.width, tilemap_json_layer.height);
  let mut tile_storage = TileStorage::empty(tilemap_size);

  // Create empty tilemap entity
  let tilemap_entity = commands.spawn_empty().id();

  for (index, tile_id) in tilemap_json_layer.data.iter().enumerate() {
    // Skip empty tiles
    if *tile_id == 0 {
      continue;
    }

    // Find the cell coordinates of the tile
    let (x, y) = get_tile_xy(index, tilemap_json_layer.width, tilemap_json_layer.height);

    // Create the logical tile
    let tile_pos = TilePos::new(x, y);
    let tile_bundle = TileBundle {
      position: tile_pos,
      texture_index: TileTextureIndex(tile_id - first_guid),
      tilemap_id: TilemapId(tilemap_entity),
      ..Default::default()
    };

    // Spawn the tile and store it
    let tile_entity = commands.spawn(tile_bundle).id();
    tile_storage.set(&tile_pos, tile_entity);
  }

  // Create the tilemap transform
  // Must be scaled as a whole, as we can increase the size of individual tiles
  let scale_x = TILE_SIZE / tilewidth as f32;
  let scale_y = TILE_SIZE / tileheight as f32;
  let mut tilemap_transform = Transform::from_scale(Vec3::new(scale_x, scale_y, z_index));
  tilemap_transform.translation.x = HALF_TILE_SIZE;
  tilemap_transform.translation.y = HALF_TILE_SIZE;
  tilemap_transform.translation.z = z_index;

  // Create the tilemap components and insert into the empty tilemap entity
  let tilemap_bundle = TilemapBundle {
    map_type: TilemapType::Square,
    grid_size: TilemapGridSize::new(tilewidth as f32, tileheight as f32),
    size: tilemap_size,
    storage: tile_storage,
    texture: TilemapTexture::Single(tileset_image_handle),
    tile_size: TilemapTileSize::new(tilewidth as f32, tileheight as f32),
    transform: tilemap_transform,
    ..Default::default()
  };
  commands.entity(tilemap_entity).insert(tilemap_bundle);
}

/** Spawns a tilemap as individual sprites, required for depth sorting. */
pub fn spawn_structure_layer(
  commands: &mut Commands,
  tilemap_json_layer: &TiledMapJsonTileLayer,
  tileset_image_handle: Handle<Image>,
  tilewidth: u32,
  tileheight: u32,
  first_guid: u32,
) {
  for (index, tile_id) in tilemap_json_layer.data.iter().enumerate() {
    // Skip empty tiles
    if *tile_id == 0 {
      continue;
    }

    // Determine sprite's bevy (x,y,z) from the index in the array
    let (x, y) = get_tile_xy(index, tilemap_json_layer.width, tilemap_json_layer.height);
    let transform = Transform::from_xyz(
      x as f32 * TILE_SIZE + HALF_TILE_SIZE,
      y as f32 * TILE_SIZE,
      get_z_from_y(y as f32 * TILE_SIZE),
    );

    // Determine where the sprite is in the texture image based on the tileId
    let rect_x = ((tile_id - first_guid) * tilewidth) as f32;
    let rect = Rect::new(
      rect_x,
      0.,
      rect_x + tilewidth as f32,
      tileheight as f32,
    );

    // Create the sprite
    let mut sprite = Sprite::from_image(tileset_image_handle.clone());
    sprite.rect = Some(rect);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    // Spawn the sprite
    commands.spawn((
      StructureTile,
      Anchor::BOTTOM_CENTER,
      sprite,
      transform,
    ));
  }
}

pub fn spawn_map(
  commands: &mut Commands,
  tilemap_json: &TiledMapJson,
  tileset_terrain_image_handle: Handle<Image>,
  tileset_structures_image_handle: Handle<Image>,
) -> Result<(), TilemapSpawnError>  {
  // Extract first guids, required to calculate json file tileId to texture array index 
  if tilemap_json.tilesets.len() < 2 {
    warn!("Map file has too few tilesets, found {:?}/2", tilemap_json.tilesets.len());
    return Err(TilemapSpawnError::TilesetsLength);
  }
  let first_guid_terrain = tilemap_json.tilesets[0].firstgid;
  let first_guid_structures = tilemap_json.tilesets[1].firstgid;

  // Terrain
  if let Some(
    TiledMapJsonLayer::Tile(terrain_layer)
  ) = tilemap_json.layers.get(map_constants::TILEMAP_LAYER_INDEX_TERRAIN) {
    spawn_tilemap_layer(
      commands,
      terrain_layer,
      tileset_terrain_image_handle.clone(),
      tilemap_json.tilewidth,
      tilemap_json.tileheight,
      first_guid_terrain,
      TERRAIN_Z_INDEX,
    );
  } else {
    warn!("Map file does not define a valid Terrain layer");
    return Err(TilemapSpawnError::MissingInvalidTerrainLayer);
  }

  // Terrain transition
  if let Some(
    TiledMapJsonLayer::Tile(terrain_transition_layer)
  ) = tilemap_json.layers.get(map_constants::TILEMAP_LAYER_INDEX_TERRAIN_TRANSITION) {
    spawn_tilemap_layer(
      commands,
      terrain_transition_layer,
      tileset_terrain_image_handle.clone(),
      tilemap_json.tilewidth,
      tilemap_json.tileheight,
      first_guid_terrain,
      TERRAIN_Z_INDEX,
    );
  } else {
    warn!("Map file does not define a valid TerrainTransition layer");
    return Err(TilemapSpawnError::MissingInvalidTerrainTransitionLayer);
  }

  // Structures bottom
  if let Some(
    TiledMapJsonLayer::Tile(structures_base_layer)
  ) = tilemap_json.layers.get(map_constants::TILEMAP_LAYER_INDEX_STRUCTURES_BASE) {
    spawn_structure_layer(
      commands,
      structures_base_layer,
      tileset_structures_image_handle.clone(),
      tilemap_json.tilewidth,
      tilemap_json.tileheight,
      first_guid_structures,
    );
  } else {
    warn!("Map file does not define a valid StructuresBottom layer");
    return Err(TilemapSpawnError::MissingInvalidStructureBottomLayer);
  }

  // Structures top
  if let Some(
    TiledMapJsonLayer::Tile(structures_top_layer)
  ) = tilemap_json.layers.get(map_constants::TILEMAP_LAYER_INDEX_STRUCTURES_TOP) {
     spawn_tilemap_layer(
      commands,
      structures_top_layer,
      tileset_structures_image_handle.clone(),
      tilemap_json.tilewidth,
      tilemap_json.tileheight,
      first_guid_structures,
      STRUCTURE_TOP_Z_INDEX,
    );
  } else {
    warn!("Map file does not define a valid StructuresTop layer");
    return Err(TilemapSpawnError::MissingInvalidStructureTopLayer);
  }

  // Done
  Ok(())
}

fn process_teleport_region(
  commands: &mut Commands,
  map_object: &TiledMapJsonObject,
  tilewidth: u32,
  tileheight: u32,
  map_height: u32,
) {
  let Some(properties) = &map_object.properties else {
    warn!("TeleportRegion missing properties");
    return;
  };

  let Some(map_code) = properties.iter().find_map(|prop| {
    if prop.name == TiledMapJsonObjectPropertyName::TeleportRegionLocation {
      return prop.value.parse::<MapCode>().ok();
    }
    None
  }) else {
    warn!("TeleportRegion missing or invalid location");
    return;
  };

  let scale_x = TILE_SIZE / tilewidth as f32;
  let scale_y = TILE_SIZE / tileheight as f32;
  let tiled_position = Vec2::new(map_object.x as f32, map_object.y as f32);
  let position = normalize_tiled_point(&tiled_position, tilewidth, tileheight, map_height);
  let bounds = Rect::new(
    position.x,
    position.y,
    position.x + map_object.width as f32 * scale_x,
    position.y - map_object.height as f32 * scale_y,
  );

  let teleport_region = TeleportRegion {
    map_code,
    bounds,
  };
  commands.spawn(teleport_region);
  info!("Spawned TeleportRegion for {:?}", map_code);
}

pub fn process_map_object(
  commands: &mut Commands,
  map_object: &TiledMapJsonObject,
  tilewidth: u32,
  tileheight: u32,
  map_height: u32,
) {
  if map_object.object_type == TiledMapJsonObjectType::TeleportRegion {
    process_teleport_region(commands, map_object, tilewidth, tileheight, map_height);
  }
}