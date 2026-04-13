use bevy::ecs::bundle::Bundle;
use bevy::ecs::children;
use bevy::ecs::component::Component;
use bevy::image::TextureAtlas;
use bevy::ui::widget::{Button, ImageNode, Text};
use bevy::ui::{AlignItems, JustifyContent, Node, px};

use crate::models::asset_ui_animation_code::AssetUiAnimationCode;
use crate::models::asset_ui_sprite_code::AssetUiSpriteCode;
use crate::plugins::ui::ui_components::{UiAnimation, UiSprite};
use crate::resources::ui_sprite_store::UiSpriteStore;

pub fn build_ui_button_rectangle(
  marker: impl Component,
  text: &str,
  ui_sprite_store: &UiSpriteStore,
) -> impl Bundle {
  let sprite_code = AssetUiSpriteCode::ButtonRectangle;
  let animation_code = AssetUiAnimationCode::Base;
  let index = ui_sprite_store.sprite_map.get_first_frame_index(&sprite_code, &animation_code).unwrap_or_default();

  let image = ui_sprite_store.image_handle.clone();
  let atlas = TextureAtlas {
    index,
    layout: ui_sprite_store.atlas_layout_handle.clone(),
  };

  (
    marker,
    UiSprite(sprite_code),
    UiAnimation(animation_code),
    Node {
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      width: px(192.),
      height: px(64.),
      ..Default::default()
    },
    ImageNode::from_atlas_image(image, atlas),
    Button,
    children![
      Text::new(text),
    ],
  )
}