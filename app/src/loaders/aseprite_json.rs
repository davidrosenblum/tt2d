use bevy::asset::{AssetLoader, AsyncReadExt};
use bevy::reflect::TypePath;

use crate::assets::aseprite_json::AsepriteJson;

#[derive(TypePath)]
pub struct AsepriteJsonLoader;

impl AssetLoader for AsepriteJsonLoader {
  type Asset = AsepriteJson;
  type Error = Box<dyn std::error::Error + Send + Sync>;
  type Settings = ();

  fn load(
    &self,
    reader: &mut dyn bevy::asset::io::Reader,
    _settings: &Self::Settings,
    _load_context: &mut bevy::asset::LoadContext,
  ) -> impl bevy::tasks::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
    Box::pin(async move {
      let mut buf = String::new();
      reader.read_to_string(&mut buf).await?;

      let parsed_json = serde_json::from_str::<Self::Asset>(&buf)?;
      Ok(parsed_json)
    })
  }

  fn extensions(&self) -> &[&str] {
    &[".json", ".animation.json"]
  }
}