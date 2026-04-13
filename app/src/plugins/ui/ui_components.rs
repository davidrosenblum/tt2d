use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};

use crate::models::asset_ui_animation_code::AssetUiAnimationCode;
use crate::models::asset_ui_sprite_code::AssetUiSpriteCode;

// Global

#[derive(Component, Deref, DerefMut)]
pub struct UiSprite(pub AssetUiSpriteCode);

#[derive(Component, Deref, DerefMut)]
pub struct UiAnimation(pub AssetUiAnimationCode);

// Main menu

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct MainMenuPlayButton;

#[derive(Component)]
pub struct MainMenuQuitButton;