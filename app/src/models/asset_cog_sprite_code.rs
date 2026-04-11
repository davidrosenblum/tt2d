use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumString)]
pub enum AssetCogSpriteCode {
  // Bossbots
  Flunky,
  
  // Lawbots
  BottomFeeder,

  // Cashbots
  ShortChange,

  // Sellbots
  ColdCaller,
}