use crate::models::asset_cog_sprite_code::AssetCogSpriteCode;
use crate::models::cog_department_code::CogDepartmentCode;

pub struct CogData {
  pub name: &'static str,
  pub sprite_code: AssetCogSpriteCode,
  pub department_code: CogDepartmentCode,
  pub tier: u32,
  pub health: u32,
  pub melee_attack: CogDataMeleeAttack,
}

pub struct CogDataMeleeAttack {
  pub damage_range: (f32, f32),
  pub cooldown: f32,
}

pub const COG_DATA_STORE: [CogData; 12] = [
  // Bossbots
  CogData {
    name: "Flunky",
    sprite_code: AssetCogSpriteCode::Flunky,
    department_code: CogDepartmentCode::Bossbot,
    tier: 1,
    health: 6,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (1., 2.),
    },
  },
  CogData {
    name: "Pencil Pusher",
    sprite_code: AssetCogSpriteCode::PencilPusher,
    department_code: CogDepartmentCode::Bossbot,
    tier: 2,
    health: 12,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (1., 4.),
    },
  },
  CogData {
    name: "Yesman",
    sprite_code: AssetCogSpriteCode::Yesman,
    department_code: CogDepartmentCode::Bossbot,
    tier: 3,
    health: 20,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (2., 5.),
    },
  },

  // Lawbots
  CogData {
    name: "Bottom Feeder",
    sprite_code: AssetCogSpriteCode::BottomFeeder,
    department_code: CogDepartmentCode::Lawbot,
    tier: 1,
    health: 6,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (1., 2.),
    },
  },
  CogData {
    name: "Blood Sucker",
    sprite_code: AssetCogSpriteCode::BloodSucker,
    department_code: CogDepartmentCode::Lawbot,
    tier: 2,
    health: 12,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (1., 4.),
    },
  },
  CogData {
    name: "Double Talker",
    sprite_code: AssetCogSpriteCode::DoubleTalker,
    department_code: CogDepartmentCode::Lawbot,
    tier: 3,
    health: 20,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (2., 5.),
    },
  },

  // Cashbots
  CogData {
    name: "Short Change",
    sprite_code: AssetCogSpriteCode::ShortChange,
    department_code: CogDepartmentCode::Cashbot,
    tier: 1,
    health: 6,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (1., 2.),
    },
  },
  CogData {
    name: "Penny Pincher",
    sprite_code: AssetCogSpriteCode::PennyPincher,
    department_code: CogDepartmentCode::Cashbot,
    tier: 2,
    health: 12,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (1., 4.),
    },
  },
  CogData {
    name: "Tightwad",
    sprite_code: AssetCogSpriteCode::Tightwad,
    department_code: CogDepartmentCode::Cashbot,
    tier: 3,
    health: 20,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (2., 5.),
    },
  },

  // Sellbots
  CogData {
    name: "Cold Caller",
    sprite_code: AssetCogSpriteCode::ColdCaller,
    department_code: CogDepartmentCode::Sellbot,
    tier: 1,
    health: 6,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (1., 2.),
    },
  },
  CogData {
    name: "Telemarketer",
    sprite_code: AssetCogSpriteCode::Telemarketer,
    department_code: CogDepartmentCode::Sellbot,
    tier: 2,
    health: 12,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (1., 4.),
    },
  },
  CogData {
    name: "Name Dropper",
    sprite_code: AssetCogSpriteCode::NameDropper,
    department_code: CogDepartmentCode::Sellbot,
    tier: 3,
    health: 20,
    melee_attack: CogDataMeleeAttack { 
      cooldown: 3.,
      damage_range: (2., 5.),
    },
  },
];