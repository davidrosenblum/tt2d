use bevy::ecs::message::Message;

use crate::assets::tiled_map_json::TiledMapJsonObject;

#[derive(Message)]
pub struct LoadedMap {
  // Not ideal to couple this TiledMapJsonObject, but its a lazy solution
  // Downside is non-map systems have to deal with cloned map data
  pub map_objects: Vec<TiledMapJsonObject>,
  pub tilewidth: u32,
  pub tileheight: u32,
  pub height: u32,
}