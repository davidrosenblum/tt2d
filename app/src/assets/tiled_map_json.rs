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
  CogSpawner,
  PlayerSpawner,
  ToonCompanionSpawner,
  ToonNpcSpawner,
}

#[derive(Clone, Deserialize)]
pub struct TiledMapJsonObjectProperty {
  pub name: TiledMapJsonObjectPropertyName,
  /** Technically these can be different data types in Tiled. App convention is string-only. */
  pub value: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TiledMapJsonObjectPropertyName {
  #[serde(rename = "department")]
  CogSpawnerDepartment,
  #[serde(rename = "tier")]
  CogSpawnerTier,
  #[serde(rename = "companion")]
  ToonCompanionSpawnerCompanion,
  #[serde(rename = "npc")]
  ToonNpcSpawnerNpc,
}