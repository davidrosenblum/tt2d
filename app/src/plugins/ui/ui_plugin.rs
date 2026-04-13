use bevy::app::{Plugin, Update};
use bevy::ecs::query::Changed;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::{If, Query, Res};
use bevy::image::TextureAtlas;
use bevy::state::condition::in_state;
use bevy::state::state::{OnEnter, OnExit};
use bevy::ui::Interaction;
use bevy::ui::widget::ImageNode;

use crate::models::asset_ui_animation_code::AssetUiAnimationCode;
use crate::plugins::ui::ui_components::{UiAnimation, UiSprite};
use crate::plugins::ui::ui_main_menu_systems::{despawn_main_menu, spawn_main_menu, update_main_menu};
use crate::resources::ui_sprite_store::UiSpriteStore;
use crate::states::app_state::AppState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    // Global
    app.add_systems(Update, update_button_animation);

    // Main menu systems
    app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
    app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    app.add_systems(Update, update_main_menu.run_if(in_state(AppState::MainMenu)));
  }
}

// Switch between different status visually (base to active)
// Assumes each state is a single animation frame (not animated)
fn update_button_animation(
  ui_button_query: Query<(&Interaction, &mut ImageNode, &UiSprite, &mut UiAnimation), Changed<Interaction>>,
  ui_sprite_store: If<Res<UiSpriteStore>>,
) {
  for (interaction, mut image_node, ui_sprite, mut ui_animation) in ui_button_query {
    let animation_code = match interaction {
      Interaction::Hovered | Interaction::Pressed => AssetUiAnimationCode::Active,
      Interaction::None => AssetUiAnimationCode::Base,
    };

    let Some(index) = ui_sprite_store.sprite_map.get_first_frame_index(ui_sprite, &animation_code) else {
      continue;
    };

    let texture_atlas = TextureAtlas {
      index,
      layout: ui_sprite_store.atlas_layout_handle.clone(),
    };
    image_node.texture_atlas = Some(texture_atlas);

    *ui_animation = UiAnimation(animation_code);

  }
}