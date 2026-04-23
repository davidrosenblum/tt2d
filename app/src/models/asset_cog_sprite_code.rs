use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumString)]
pub enum AssetCogSpriteCode {
  // Bossbots
  Flunky,
  PencilPusher,
  Yesman,
  
  // Lawbots
  BottomFeeder,
  BloodSucker,
  DoubleTalker,

  // Cashbots
  ShortChange,
  PennyPincher,
  Tightwad,

  // Sellbots
  ColdCaller,
  Telemarketer,
  NameDropper,
}