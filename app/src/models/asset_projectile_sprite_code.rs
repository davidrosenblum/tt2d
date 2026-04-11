use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumString)]
pub enum AssetProjectileSpriteCode {
  // Cog
  BowTie,
  Gear,
  GolfBall,

  // Toon
  BirthdayCake,
}