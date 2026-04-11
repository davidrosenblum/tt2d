use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};

use crate::models::asset_cog_animation_code::AssetCogAnimationCode;

#[derive(Component, Deref, DerefMut)]
pub struct CogAnimation(pub AssetCogAnimationCode);