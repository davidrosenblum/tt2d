use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup};
use bevy::image::ImagePlugin;
use bevy::window::{Window, WindowPlugin};
use bevy_ecs_tilemap::TilemapPlugin;

use crate::constants::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use crate::plugins::asset::asset_plugin::AssetPlugin;
use crate::plugins::cog::cog_plugin::CogPlugin;
use crate::plugins::core::core_plugin::CorePlugin;
use crate::plugins::map::map_plugin::MapPlugin;
use crate::plugins::profile::profile_plugin::ProfilePlugin;
use crate::plugins::toon::toon_plugin::ToonPlugin;
use crate::plugins::ui::ui_plugin::UiPlugin;

mod constants;
mod assets;
mod components;
mod data;
mod errors;
mod messages;
mod models;
mod loaders;
mod plugins;
mod resources;
mod states;
mod utils;

fn main() {
  let default_plugins = DefaultPlugins
    .set(WindowPlugin {
      primary_window: Some(Window {
        title: "Toontown 2D".into(),
        resolution: (VIEWPORT_WIDTH, VIEWPORT_HEIGHT).into(),
        ..Default::default()
      }),
      ..Default::default()
    })
    .set(ImagePlugin::default_nearest());

  let mut app = App::new();
  app.add_plugins(default_plugins);
  app.add_plugins(TilemapPlugin);
  app.add_plugins(AssetPlugin);
  app.add_plugins(CogPlugin);
  app.add_plugins(CorePlugin);
  app.add_plugins(MapPlugin);
  app.add_plugins(ProfilePlugin);
  app.add_plugins(ToonPlugin);
  app.add_plugins(UiPlugin);
  app.run();
}
