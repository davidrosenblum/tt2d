use bevy::app::{Plugin, Startup};

use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::ecs::system::Commands;
use bevy::state::app::AppExtStates;

use crate::constants::VIEWPORT_HEIGHT;
use crate::messages::load_map_requested::LoadMapRequested;
use crate::messages::loaded_map::LoadedMap;
use crate::messages::save_profile_requested::SaveProfileRequested;
use crate::messages::saved_profile::SavedProfile;
use crate::states::app_state::AppState;

pub struct CorePlugin;

impl Plugin for CorePlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_message::<LoadMapRequested>();
    app.add_message::<LoadedMap>();
    app.add_message::<SaveProfileRequested>();
    app.add_message::<SavedProfile>();

    app.init_state::<AppState>();

    app.add_systems(Startup, spawn_camera);
  }
}

fn spawn_camera(
  mut commands: Commands,
) {
  commands.spawn((
    Camera2d,
    Projection::Orthographic(OrthographicProjection {
      scaling_mode: ScalingMode::FixedVertical { viewport_height: VIEWPORT_HEIGHT as f32 },
      ..OrthographicProjection::default_2d()
    }),
  ));
}