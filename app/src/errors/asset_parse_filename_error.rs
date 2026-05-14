#[derive(Debug)]
pub enum AssetParseFilenameError {
  Malformed,
  InvalidSpriteCode,
  InvalidAnimationCode,
  InvalidIndex,
}