use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};

use crate::models::asset_toon_sprite_code::AssetToonSpriteCode;

#[derive(Component, Deref, DerefMut)]
pub struct ToonSprite(pub AssetToonSpriteCode);