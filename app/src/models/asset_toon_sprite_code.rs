use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumString)]
pub enum AssetToonSpriteCode {
  // Player
  Player,

  // Companions
  Comet,
  Dizzy,

  // Npcs
  Flippy,
  ProfessorPete,
  StickyLou,
}