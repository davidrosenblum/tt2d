use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumString)]
#[strum(serialize_all = "camelCase")]
pub enum AssetToonAnimationCode {
  Idle,
  Walk,
  Attack,
}