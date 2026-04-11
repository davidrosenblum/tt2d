use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};

use crate::models::asset_toon_animation_code::AssetToonAnimationCode;

#[derive(Component, Deref, DerefMut)]
pub struct ToonAnimation(pub AssetToonAnimationCode);