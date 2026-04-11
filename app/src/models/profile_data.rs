use serde::{Deserialize, Serialize};

use crate::models::map_code::MapCode;

#[derive(Clone, Deserialize, Serialize)]
pub struct ProfileData {
  pub version: u32,
  pub toons: Vec<ProfileDataToon>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ProfileDataToon {
  pub map_code: MapCode,
}

impl Default for ProfileData {
  fn default() -> Self {
    Self {
      version: 1,
      toons: vec![ProfileDataToon::default()],
    }
  }
}

impl Default for ProfileDataToon {
  fn default() -> Self {
    Self {
      map_code: MapCode::TtcPlayground,
    }
  }
}