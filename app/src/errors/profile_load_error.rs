#[derive(Debug)]
pub enum ProfileLoadError {
  DeserializeFailed,
  FileNotFound,
  FileReadFailed,
}