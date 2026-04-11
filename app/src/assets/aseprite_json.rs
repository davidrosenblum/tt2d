use bevy::asset::Asset;
use bevy::platform::collections::HashMap;
use bevy::reflect::TypePath;
use serde::Deserialize;

#[derive(Asset, Deserialize, TypePath)]
pub struct AsepriteJson {
  pub frames: HashMap<String, AsepriteJsonFrame>,
  pub meta: AsepriteJsonMeta,
}

#[derive(Deserialize)]
pub struct AsepriteJsonFrame {
  pub duration: u32,
  pub frame: AsepriteJsonFrameFrame,
}

#[derive(Deserialize)]
pub struct AsepriteJsonFrameFrame {
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

#[derive(Deserialize)]
pub struct AsepriteJsonMeta {
  pub size: AsepriteJsonMetaSize,
}

#[derive(Deserialize)]
pub struct AsepriteJsonMetaSize {
  pub w: u32,
  pub h: u32,
}