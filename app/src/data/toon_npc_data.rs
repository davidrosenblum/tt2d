use crate::models::asset_toon_sprite_code::AssetToonSpriteCode;
use crate::models::toon_npc_code::ToonNpcCode;

pub struct ToonNpcData {
  pub name: &'static str,
  pub npc_code: ToonNpcCode,
  pub sprite_code: AssetToonSpriteCode,
}

pub const TOON_NPC_DATA_STORE: [ToonNpcData; 3] = [
  // Ttc
  ToonNpcData {
    name: "Flippy",
    npc_code: ToonNpcCode::Flippy,
    sprite_code: AssetToonSpriteCode::Flippy,
  },
  ToonNpcData {
    name: "Professor Pete",
    npc_code: ToonNpcCode::ProfessorPete,
    sprite_code: AssetToonSpriteCode::ProfessorPete,
  },
  ToonNpcData {
    name: "Stick Lou",
    npc_code: ToonNpcCode::StickyLou,
    sprite_code: AssetToonSpriteCode::StickyLou,
  },
];