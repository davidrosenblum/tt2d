use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};

use crate::models::asset_cog_sprite_code::AssetCogSpriteCode;

#[derive(Component, Deref, DerefMut)]
pub struct CogSprite(pub AssetCogSpriteCode);