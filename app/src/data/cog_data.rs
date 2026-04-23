use crate::models::asset_cog_sprite_code::AssetCogSpriteCode;
use crate::models::cog_department_code::CogDepartmentCode;

pub struct CogData {
  pub name: &'static str,
  pub sprite_code: AssetCogSpriteCode,
  pub department_code: CogDepartmentCode,
  pub tier: u32,
}

pub const COG_DATA_STORE: [CogData; 12] = [
  // Bossbots
  CogData {
    name: "Flunky",
    sprite_code: AssetCogSpriteCode::Flunky,
    department_code: CogDepartmentCode::Bossbot,
    tier: 1,
  },
  CogData {
    name: "Pencil Pusher",
    sprite_code: AssetCogSpriteCode::PencilPusher,
    department_code: CogDepartmentCode::Bossbot,
    tier: 2,
  },
  CogData {
    name: "Yesman",
    sprite_code: AssetCogSpriteCode::Yesman,
    department_code: CogDepartmentCode::Bossbot,
    tier: 3,
  },

  // Lawbots
  CogData {
    name: "Bottom Feeder",
    sprite_code: AssetCogSpriteCode::BottomFeeder,
    department_code: CogDepartmentCode::Lawbot,
    tier: 1,
  },
  CogData {
    name: "Blood Sucker",
    sprite_code: AssetCogSpriteCode::BloodSucker,
    department_code: CogDepartmentCode::Lawbot,
    tier: 2,
  },
  CogData {
    name: "Double Talker",
    sprite_code: AssetCogSpriteCode::DoubleTalker,
    department_code: CogDepartmentCode::Lawbot,
    tier: 3,
  },

  // Cashbots
  CogData {
    name: "Short Change",
    sprite_code: AssetCogSpriteCode::ShortChange,
    department_code: CogDepartmentCode::Cashbot,
    tier: 1,
  },
  CogData {
    name: "Short Change",
    sprite_code: AssetCogSpriteCode::PennyPincher,
    department_code: CogDepartmentCode::Cashbot,
    tier: 2,
  },
  CogData {
    name: "Short Change",
    sprite_code: AssetCogSpriteCode::Tightwad,
    department_code: CogDepartmentCode::Cashbot,
    tier: 3,
  },

  // Sellbots
  CogData {
    name: "Cold Caller",
    sprite_code: AssetCogSpriteCode::ColdCaller,
    department_code: CogDepartmentCode::Sellbot,
    tier: 1,
  },
  CogData {
    name: "Telemarketer",
    sprite_code: AssetCogSpriteCode::Telemarketer,
    department_code: CogDepartmentCode::Sellbot,
    tier: 2,
  },
  CogData {
    name: "Name Dropper",
    sprite_code: AssetCogSpriteCode::NameDropper,
    department_code: CogDepartmentCode::Sellbot,
    tier: 3,
  },
];