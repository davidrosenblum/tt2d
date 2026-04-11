use crate::models::asset_cog_sprite_code::AssetCogSpriteCode;
use crate::models::cog_department_code::CogDepartmentCode;

pub struct CogData {
  pub name: &'static str,
  pub sprite_code: AssetCogSpriteCode,
  pub department_code: CogDepartmentCode,
  pub tier: u32,
}

pub const COG_DATA_STORE: [CogData; 4] = [
  // Bossbots
  CogData {
    name: "Flunky",
    sprite_code: AssetCogSpriteCode::Flunky,
    department_code: CogDepartmentCode::Bossbot,
    tier: 1,
  },

  // Lawbots
  CogData {
    name: "Bottom Feeder",
    sprite_code: AssetCogSpriteCode::BottomFeeder,
    department_code: CogDepartmentCode::Lawbot,
    tier: 1,
  },

  // Cashbots
  CogData {
    name: "Short Change",
    sprite_code: AssetCogSpriteCode::ShortChange,
    department_code: CogDepartmentCode::Cashbot,
    tier: 1,
  },

  // Sellbots
  CogData {
    name: "Cold Caller",
    sprite_code: AssetCogSpriteCode::ColdCaller,
    department_code: CogDepartmentCode::Sellbot,
    tier: 1,
  },
];