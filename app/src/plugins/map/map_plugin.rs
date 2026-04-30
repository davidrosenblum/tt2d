use bevy::app::{Plugin, Update};
use bevy::asset::{AssetServer, Assets};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::query::With;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::{Commands, If, Query, Res, ResMut};
use bevy::image::Image;
use bevy::log::{info, warn};
use bevy::state::condition::in_state;
use bevy::state::state::{NextState, OnExit};
use bevy_ecs_tilemap::tiles::TileStorage;

use crate::assets::tiled_map_json::{TiledMapJson, TiledMapJsonLayer};
use crate::components::structure_tile::StructureTile;
use crate::data::map_data::MAP_DATA_STORE;
use crate::messages::load_map_requested::LoadMapRequested;
use crate::messages::loaded_map::{LoadedMap};
use crate::models::map_code::MapCode;
use crate::plugins::map::map_constants::{BBHQ_CEO_TILEMAP_PATH, BBHQ_COURTYARD_TILEMAP_PATH, BBHQ_GOLFCOURSE_TILEMAP_PATH, BBHQ_LOBBY_TILEMAP_PATH, BBHQ_STRUCTURES_IMAGE_PATH, BBHQ_TERRAIN_IMAGE_PATH, BR_PLAYGROUND_TILEMAP_PATH, BR_STREETS_TILEMAP_PATH, BR_TILESET_STRUCTURES_IMAGE_PATH, BR_TILESET_TERRAIN_IMAGE_PATH, CBHQ_CFO_TILEMAP_PATH, CBHQ_COURTYARD_TILEMAP_PATH, CBHQ_LOBBY_TILEMAP_PATH, CBHQ_MINT_TILEMAP_PATH, CBHQ_STRUCTURES_IMAGE_PATH, CBHQ_TERRAIN_IMAGE_PATH, COGBUILDING_BB_TILEMAP_PATH, COGBUILDING_CB_TILEMAP_PATH, COGBUILDING_LB_TILEMAP_PATH, COGBUILDING_SB_TILEMAP_PATH, COGBUILDING_TILESET_STRUCTURES_IMAGE_PATH, COGBUILDING_TILESET_TERRAIN_IMAGE_PATH, DD_PLAYGROUND_TILEMAP_PATH, DD_STREETS_TILEMAP_PATH, DD_TILESET_STRUCTURES_IMAGE_PATH, DD_TILESET_TERRAIN_IMAGE_PATH, DDL_PLAYGROUND_TILEMAP_PATH, DDL_STREETS_TILEMAP_PATH, DDL_TILESET_STRUCTURES_IMAGE_PATH, DDL_TILESET_TERRAIN_IMAGE_PATH, DG_PLAYGROUND_TILEMAP_PATH, DG_STREETS_TILEMAP_PATH, DG_TILESET_STRUCTURES_IMAGE_PATH, DG_TILESET_TERRAIN_IMAGE_PATH, LBHQ_CJ_TILEMAP_PATH, LBHQ_COURTYARD_TILEMAP_PATH, LBHQ_DAOFFICE_TILEMAP_PATH, LBHQ_LOBBY_TILEMAP_PATH, LBHQ_STRUCTURES_IMAGE_PATH, LBHQ_TERRAIN_IMAGE_PATH, MML_PLAYGROUND_TILEMAP_PATH, MML_STREETS_TILEMAP_PATH, MML_TILESET_STRUCTURES_IMAGE_PATH, MML_TILESET_TERRAIN_IMAGE_PATH, SBHQ_COURTYARD_TILEMAP_PATH, SBHQ_FACTORY_TILEMAP_PATH, SBHQ_LOBBY_TILEMAP_PATH, SBHQ_STRUCTURES_IMAGE_PATH, SBHQ_TERRAIN_IMAGE_PATH, SBHQ_VP_TILEMAP_PATH, TILEMAP_LAYER_INDEX_OBJECTS, TTC_PLAYGROUND_TILEMAP_PATH, TTC_STREETS_TILEMAP_PATH, TTC_TILESET_STRUCTURES_IMAGE_PATH, TTC_TILESET_TERRAIN_IMAGE_PATH};
use crate::plugins::map::map_utils::{get_map_bounds, spawn_map};
use crate::resources::map_context::MapContext;
use crate::resources::map_load_tracker::MapLoadTracker;
use crate::states::app_state::AppState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(OnExit(AppState::InGame), despawn_on_exit);
    app.add_systems(Update, poll_load_map_requested);
    app.add_systems(Update, check_map_loaded.run_if(in_state(AppState::LoadingMap)));
  }
}

fn despawn_on_exit(
  mut commands: Commands,
  tilemaps_query: Query<(Entity, &mut TileStorage)>,
  structure_tiles_query: Query<Entity, With<StructureTile>>,
) {
  // Cleanup tilemaps and nested tilemap tiles (tiles in tilemaps)
  for (tilemap_entity, mut tile_storage) in tilemaps_query {
    for tile in tile_storage.drain() {
      commands.entity(tile).despawn();
    }
    commands.entity(tilemap_entity).despawn();
  }

  // Cleanup non-tilemap tile sprites (tiles not in tilemaps)
  for tile_entity in structure_tiles_query {
    commands.entity(tile_entity).despawn();
  }
}

fn poll_load_map_requested(
  mut commands: Commands,
  mut reader: MessageReader<LoadMapRequested>,
  existing_tracker: Option<Res<MapLoadTracker>>,
  asset_server: Res<AssetServer>,
  mut next_state: ResMut<NextState<AppState>>,
) {
  // Only load last event
  let Some(message) = reader.read().last() else {
    return;
  };

  // Bail out if already loading a map
  if existing_tracker.is_some() {
    warn!("Detected attempting to load a map while already loading a map");
    commands.remove_resource::<MapLoadTracker>();
    return;
  }

  // Find the map data for the map code
  let Some(map_data) = MAP_DATA_STORE.iter().find(|map| map.map_code == message.map_code) else {
    warn!("Missing map data for {:?}", message.map_code);
    commands.remove_resource::<MapLoadTracker>();
    return;
  };

  // Figure out which terrain tilset and start loading
  let tileset_terrain_image_path = match message.map_code {
    MapCode::TtcPlayground | MapCode::TtcStreet => TTC_TILESET_TERRAIN_IMAGE_PATH,
    MapCode::DdPlayground | MapCode::DdStreet => DD_TILESET_TERRAIN_IMAGE_PATH,
    MapCode::DgPlayground | MapCode::DgStreet => DG_TILESET_TERRAIN_IMAGE_PATH,
    MapCode::MmlPlayground | MapCode::MmlStreet => MML_TILESET_TERRAIN_IMAGE_PATH,
    MapCode::BrPlayground | MapCode::BrStreet => BR_TILESET_TERRAIN_IMAGE_PATH,
    MapCode::DdlPlayground | MapCode::DdlStreet => DDL_TILESET_TERRAIN_IMAGE_PATH,
    MapCode::CogBuildingBb | MapCode::CogBuildingLb | MapCode::CogBuildingCb | MapCode::CogBuildingSb => COGBUILDING_TILESET_TERRAIN_IMAGE_PATH,
    MapCode::SbhqCourtyard | MapCode::SbhqFactory | MapCode::SbhqLobby | MapCode::SbhqVp => SBHQ_TERRAIN_IMAGE_PATH,
    MapCode::CbhqCourtyard | MapCode::CbhqMint | MapCode::CbhqLobby | MapCode::CbhqCfo => CBHQ_TERRAIN_IMAGE_PATH,
    MapCode::LbhqCourtyard | MapCode::LbhqDaOffice | MapCode::LbhqLobby | MapCode::LbhqCj => LBHQ_TERRAIN_IMAGE_PATH,
    MapCode::BbhqCourtyard | MapCode::BbhqGolfCourse | MapCode::BbhqLobby | MapCode::BbhqCeo => BBHQ_TERRAIN_IMAGE_PATH,
  };
  let tileset_terrain_image_handle = asset_server.load::<Image>(tileset_terrain_image_path);

  // Figure out which structure tileset and starting loading
  let tileset_structure_image_path = match message.map_code {
    MapCode::TtcPlayground | MapCode::TtcStreet => TTC_TILESET_STRUCTURES_IMAGE_PATH,
    MapCode::DdPlayground | MapCode::DdStreet => DD_TILESET_STRUCTURES_IMAGE_PATH,
    MapCode::DgPlayground | MapCode::DgStreet => DG_TILESET_STRUCTURES_IMAGE_PATH,
    MapCode::MmlPlayground | MapCode::MmlStreet => MML_TILESET_STRUCTURES_IMAGE_PATH,
    MapCode::BrPlayground | MapCode::BrStreet => BR_TILESET_STRUCTURES_IMAGE_PATH,
    MapCode::DdlPlayground | MapCode::DdlStreet => DDL_TILESET_STRUCTURES_IMAGE_PATH,
    MapCode::CogBuildingBb | MapCode::CogBuildingLb | MapCode::CogBuildingCb | MapCode::CogBuildingSb => COGBUILDING_TILESET_STRUCTURES_IMAGE_PATH,
    MapCode::SbhqCourtyard | MapCode::SbhqFactory | MapCode::SbhqLobby | MapCode::SbhqVp => SBHQ_STRUCTURES_IMAGE_PATH,
    MapCode::CbhqCourtyard | MapCode::CbhqMint | MapCode::CbhqLobby | MapCode::CbhqCfo => CBHQ_STRUCTURES_IMAGE_PATH,
    MapCode::LbhqCourtyard | MapCode::LbhqDaOffice | MapCode::LbhqLobby | MapCode::LbhqCj => LBHQ_STRUCTURES_IMAGE_PATH,
    MapCode::BbhqCourtyard | MapCode::BbhqGolfCourse | MapCode::BbhqLobby | MapCode::BbhqCeo => BBHQ_STRUCTURES_IMAGE_PATH,
  };
  let tileset_structure_image_handle = asset_server.load::<Image>(tileset_structure_image_path);

  // Figure out which tilemap and start loading
  let tilemap_json_path = match message.map_code {
    MapCode::TtcPlayground => TTC_PLAYGROUND_TILEMAP_PATH,
    MapCode::TtcStreet => TTC_STREETS_TILEMAP_PATH,
    MapCode::DdPlayground => DD_PLAYGROUND_TILEMAP_PATH,
    MapCode::DdStreet => DD_STREETS_TILEMAP_PATH,
    MapCode::DgPlayground => DG_PLAYGROUND_TILEMAP_PATH,
    MapCode::DgStreet => DG_STREETS_TILEMAP_PATH,
    MapCode::MmlPlayground => MML_PLAYGROUND_TILEMAP_PATH,
    MapCode::MmlStreet => MML_STREETS_TILEMAP_PATH,
    MapCode::BrPlayground => BR_PLAYGROUND_TILEMAP_PATH,
    MapCode::BrStreet => BR_STREETS_TILEMAP_PATH,
    MapCode::DdlPlayground => DDL_PLAYGROUND_TILEMAP_PATH,
    MapCode::DdlStreet => DDL_STREETS_TILEMAP_PATH,
    MapCode::CogBuildingBb => COGBUILDING_BB_TILEMAP_PATH,
    MapCode::CogBuildingLb => COGBUILDING_LB_TILEMAP_PATH,
    MapCode::CogBuildingCb => COGBUILDING_CB_TILEMAP_PATH,
    MapCode::CogBuildingSb => COGBUILDING_SB_TILEMAP_PATH,
    MapCode::SbhqCourtyard => SBHQ_COURTYARD_TILEMAP_PATH,
    MapCode::SbhqFactory => SBHQ_FACTORY_TILEMAP_PATH,
    MapCode::SbhqLobby => SBHQ_LOBBY_TILEMAP_PATH,
    MapCode::SbhqVp => SBHQ_VP_TILEMAP_PATH,
    MapCode::CbhqCourtyard => CBHQ_COURTYARD_TILEMAP_PATH,
    MapCode::CbhqMint => CBHQ_MINT_TILEMAP_PATH,
    MapCode::CbhqLobby => CBHQ_LOBBY_TILEMAP_PATH,
    MapCode::CbhqCfo => CBHQ_CFO_TILEMAP_PATH,
    MapCode::LbhqCourtyard => LBHQ_COURTYARD_TILEMAP_PATH,
    MapCode::LbhqDaOffice => LBHQ_DAOFFICE_TILEMAP_PATH,
    MapCode::LbhqLobby => LBHQ_LOBBY_TILEMAP_PATH,
    MapCode::LbhqCj => LBHQ_CJ_TILEMAP_PATH,
    MapCode::BbhqCourtyard => BBHQ_COURTYARD_TILEMAP_PATH,
    MapCode::BbhqGolfCourse => BBHQ_GOLFCOURSE_TILEMAP_PATH,
    MapCode::BbhqLobby => BBHQ_LOBBY_TILEMAP_PATH,
    MapCode::BbhqCeo => BBHQ_CEO_TILEMAP_PATH,
  };
  let tilemap_json_handle = asset_server.load::<TiledMapJson>(tilemap_json_path);

  // Track the load
  let map_load_tracker = MapLoadTracker {
    map_code: message.map_code,
    map_data: map_data.clone(),
    tileset_terrain_image_handle,
    tileset_structure_image_handle,
    tilemap_json_handle,
  };
  commands.insert_resource(map_load_tracker);

  // Go to loading screen
  next_state.set(AppState::LoadingMap);
  info!("Loading map {:?}", message.map_code);
}

fn check_map_loaded(
  mut commands: Commands,
  mut loaded_map_message_witer: MessageWriter<LoadedMap>,
  tracker: If<Res<MapLoadTracker>>,
  tiled_map_json_assets: Res<Assets<TiledMapJson>>,
  asset_server: Res<AssetServer>,
  mut next_state: ResMut<NextState<AppState>>,
) {
  // Check if still loading the tileset images
  if !asset_server.is_loaded(tracker.tileset_terrain_image_handle.id()) {
    // Still loading
    return;
  }
  if !asset_server.is_loaded(tracker.tileset_structure_image_handle.id()) {
    // Still loading
    return;
  }

  // Check if still loading the tilemap json
  if !asset_server.is_loaded(tracker.tilemap_json_handle.id()) {
    // Still loading
    return;
  }

  // Get the tilemap json data to build the map
  let tilemap_json = tiled_map_json_assets.get(tracker.tilemap_json_handle.id())
    .expect("Failed to load tilemap file");

  // Spawn the map
  info!("Map loading, spawning map for {:?}", tracker.map_code);
  spawn_map(
    &mut commands,
    tilemap_json,
    tracker.tileset_terrain_image_handle.clone(),
    tracker.tileset_structure_image_handle.clone(),
  ).expect("Map file is corrupted");

  // Save the map context
  let map_context = MapContext {
    map_code: tracker.map_code,
    map_data: tracker.map_data,
    bounds: get_map_bounds(tilemap_json),
  };
  commands.insert_resource(map_context);

  // Extract map objects
  let map_objects = match tilemap_json.layers.get(TILEMAP_LAYER_INDEX_OBJECTS) {
     Some(TiledMapJsonLayer::Object(object_layer)) => object_layer.objects.clone(),
     _ => {
      warn!("Map file is missing object layer");
      vec![]
     },
  };

  // Signal map loaded
  let loaded_map_message = LoadedMap {
    map_objects,
    tilewidth: tilemap_json.tilewidth,
    tileheight: tilemap_json.tileheight,
    height: tilemap_json.height,
  };
  loaded_map_message_witer.write(loaded_map_message);

  // Show the game
  next_state.set(AppState::InGame);
}