use crate::models::asset_toon_sprite_code::AssetToonSpriteCode;
use crate::models::toon_companion_code::ToonCompanionCode;

pub struct ToonCompanionData {
  pub name: &'static str,
  pub companion_code: ToonCompanionCode,
  pub sprite_code: AssetToonSpriteCode,
}

pub const TOON_COMANION_DATA_STORE: [ToonCompanionData; 2] = [
  ToonCompanionData {
    name: "Comet",
    companion_code: ToonCompanionCode::Comet,
    sprite_code: AssetToonSpriteCode::Comet,
  },
  ToonCompanionData {
    name: "Dizzy",
    companion_code: ToonCompanionCode::Dizzy,
    sprite_code: AssetToonSpriteCode::Dizzy,
  },
];