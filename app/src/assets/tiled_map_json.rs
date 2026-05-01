use bevy::asset::Asset;
use bevy::reflect::TypePath;
use serde::Deserialize;

#[derive(Asset, Deserialize, TypePath)]
pub struct TiledMapJson {
  pub tilewidth: u32,
  pub tileheight: u32,
  pub width: u32,
  pub height: u32,
  pub tilesets: Vec<TiledMapJsonTilesets>,
  pub layers: Vec<TiledMapJsonLayer>,
}

#[derive(Deserialize)]
pub struct TiledMapJsonTilesets {
  pub firstgid: u32,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum TiledMapJsonLayer {
  #[serde(rename = "objectgroup")]
  Object(TiledMapJsonObjectLayer),
  #[serde(rename = "tilelayer")]
  Tile(TiledMapJsonTileLayer),
}

#[derive(Deserialize)]
pub struct TiledMapJsonTileLayer {
  pub data: Vec<u32>,
  pub width: u32,
  pub height: u32,
}

#[derive(Clone, Deserialize)]
pub struct TiledMapJsonObjectLayer {
  pub objects: Vec<TiledMapJsonObject>,
}

#[derive(Clone, Deserialize)]
pub struct TiledMapJsonObject {
  #[serde(rename = "type")]
  pub object_type: TiledMapJsonObjectType,
  pub x: u32,
  pub y: u32,
  pub properties: Option<Vec<TiledMapJsonObjectProperty>>,
}

/** 1:1 with Tiled custom classes. */
#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TiledMapJsonObjectType {
  // Spawners
  CogSpawner,
  PlayerSpawner,
  ToonCompanionSpawner,
  ToonNpcSpawner,
  // Regions
  CogRegion,
  TeleportRegion,
}

#[derive(Clone, Deserialize)]
pub struct TiledMapJsonObjectProperty {
  pub name: TiledMapJsonObjectPropertyName,
  /** Technically these can be different data types in Tiled. App convention is string-only. */
  pub value: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TiledMapJsonObjectPropertyName {
  // CogSpawner
  #[serde(rename = "cog_department")]
  CogSpawnerDepartment,
  #[serde(rename = "cog_facing")]
  CogSpawnerFacing,
  #[serde(rename = "cog_tier")]
  CogSpawnerTier,

  // ToonCompanionSpawner
  #[serde(rename = "companion")]
  ToonCompanionSpawnerCompanion,
  #[serde(rename = "companion_facing")]
  ToonCompanionSpawnerCompanionFacing,

  // ToonNpcSpawner
  #[serde(rename = "npc")]
  ToonNpcSpawnerNpc,
  #[serde(rename = "npc_facing")]
  ToonNpcSpawnerFacing,

  // CogRegion
  #[serde(rename = "cog_region_count")]
  CogRegionCount,
  #[serde(rename = "cog_region_department")]
  CogRegionDepartment,
  #[serde(rename = "cog_region_difficult")]
  CogRegionDifficulty,
  #[serde(rename = "cog_region_tier_min")]
  CogRegionTierMin,
  #[serde(rename = "cog_region_tier_max")]
  CogRegionTierMax,

  // TeleportRegion
  #[serde(rename = "tp_location")]
  TeleportRegionLocation,
}