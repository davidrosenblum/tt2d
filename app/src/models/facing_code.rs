use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, EnumString)]
pub enum FacingCode {
  Left,
  #[default]
  Right,
}