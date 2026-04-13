use bevy::app::AppExit;
use bevy::color::palettes::css::BLACK;
use bevy::ecs::children;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Commands, Query, Res};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node, percent};

use crate::messages::load_map_requested::LoadMapRequested;
use crate::models::map_code::MapCode;
use crate::plugins::ui::ui_components::{MainMenuPlayButton, MainMenuQuitButton, MainMenuRoot};
use crate::plugins::ui::ui_utils::build_ui_button_rectangle;
use crate::resources::profile_context::ProfileContext;
use crate::resources::ui_sprite_store::UiSpriteStore;

pub fn spawn_main_menu(
  mut commands: Commands,
  ui_sprite_store: Res<UiSpriteStore>,
) {
  commands.spawn((
    MainMenuRoot,
    Node {
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      flex_direction: FlexDirection::Column,
      width: percent(100.),
      height: percent(100.),
      ..Default::default()
    },
    BackgroundColor(BLACK.into()),
    children![
      build_ui_button_rectangle(MainMenuPlayButton, "Play", &ui_sprite_store),
      build_ui_button_rectangle(MainMenuQuitButton, "Quit", &ui_sprite_store),
    ],
  ));
}

pub fn despawn_main_menu(
  mut commands: Commands,
  query: Query<Entity, With<MainMenuRoot>>,
) {
  if let Ok(entity) = query.single() {
    commands.entity(entity).despawn();
  }
}

pub fn update_main_menu(
  play_button_query: Query<&Interaction, (With<MainMenuPlayButton>, Changed<Interaction>)>,
  quit_button_query: Query<&Interaction, (With<MainMenuQuitButton>, Changed<Interaction>)>,
  profile_context: Res<ProfileContext>,
  mut load_map_writer: MessageWriter<LoadMapRequested>,
  mut app_exit_writer: MessageWriter<AppExit>,
) {
  if let Ok(interaction) = play_button_query.single() {
    if *interaction == Interaction::Pressed {
      // Load map when clicking play button, this will move to map loading app state
      // TODO multiple toon saves in the future
      let map_code = profile_context.toons.first().map(|toon| toon.map_code).unwrap_or_else(|| MapCode::TtcPlayground);
      let message = LoadMapRequested { map_code };
      load_map_writer.write(message);
    }
  }

  if let Ok(interaction) = quit_button_query.single() {
    if *interaction == Interaction::Pressed {
      app_exit_writer.write(AppExit::Success);
    }
  }
}