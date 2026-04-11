use bevy::ecs::message::Message;

use crate::models::map_code::MapCode;

#[derive(Message)]
pub struct LoadMapRequested {
  pub map_code: MapCode,
}