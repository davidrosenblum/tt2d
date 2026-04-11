#[derive(Debug)]
pub enum ProfileSaveError {
  SerializeFailed,
  FileWriteFailed,
}