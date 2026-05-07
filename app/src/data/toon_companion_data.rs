use crate::models::asset_toon_sprite_code::AssetToonSpriteCode;
use crate::models::toon_companion_code::ToonCompanionCode;

pub struct ToonCompanionData {
  pub name: &'static str,
  pub companion_code: ToonCompanionCode,
  pub sprite_code: AssetToonSpriteCode,
  pub health: u32,
  pub melee_attack: ToonCompanionDataMeleeAttack,
}

pub struct ToonCompanionDataMeleeAttack {
  pub cooldown: f32,
  pub damage_range: (f32, f32),
}

pub const TOON_COMANION_DATA_STORE: [ToonCompanionData; 2] = [
  ToonCompanionData {
    name: "Comet",
    companion_code: ToonCompanionCode::Comet,
    sprite_code: AssetToonSpriteCode::Comet,
    health: 25,
    melee_attack: ToonCompanionDataMeleeAttack {
      cooldown: 2.5,
      damage_range: (1., 3.),
    },
  },
  ToonCompanionData {
    name: "Dizzy",
    companion_code: ToonCompanionCode::Dizzy,
    sprite_code: AssetToonSpriteCode::Dizzy,
    health: 25,
    melee_attack: ToonCompanionDataMeleeAttack {
      cooldown: 2.5,
      damage_range: (1., 3.),
    },
  },
];