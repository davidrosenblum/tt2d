use std::io::ErrorKind;

use bevy::app::{Plugin, Update};
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::{Commands, If, Res, ResMut};
use bevy::log::{error, info};
use bevy::state::condition::in_state;
use bevy::state::state::{NextState, OnEnter};
use bevy::tasks::{IoTaskPool, Task, block_on};
use bevy::tasks::futures_lite::future;

use crate::errors::profile_load_error::ProfileLoadError;
use crate::errors::profile_save_error::ProfileSaveError;
use crate::messages::save_profile_requested::SaveProfileRequested;
use crate::messages::saved_profile::SavedProfile;
use crate::models::profile_data::ProfileData;
use crate::plugins::profile::profile_constants::PROFILE_PATH;
use crate::resources::profile_context::ProfileContext;
use crate::resources::profile_load_tracker::ProfileLoadTracker;
use crate::resources::profile_save_tracker::ProfileSaveTracker;
use crate::states::app_state::AppState;

pub struct ProfilePlugin;

impl Plugin for ProfilePlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(OnEnter(AppState::LoadingProfile), load_profile);
    app.add_systems(Update, check_profile_loaded.run_if(in_state(AppState::LoadingProfile)));

    app.add_systems(Update, poll_save_profile_requested);
    app.add_systems(Update, check_profile_saved);
  }
}

fn load_profile(
  mut commands: Commands,
) {
  // Load the profile file in a separate thread
  let task: Task<Result<ProfileData, ProfileLoadError>> = IoTaskPool::get().spawn(async move {
    // Read the file
    let json = std::fs::read_to_string(PROFILE_PATH)
      .map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
          return ProfileLoadError::FileNotFound
        }
        ProfileLoadError::FileReadFailed
      })?;

    // Parse the file
    let profile_data = serde_json::from_str::<ProfileData>(&json)
      .map_err(|_| ProfileLoadError::DeserializeFailed)?;

    // File load complete
    Ok(profile_data)
  });

  // Create the tracker
  commands.insert_resource(ProfileLoadTracker { task });
}

fn check_profile_loaded(
  mut commands: Commands,
  mut tracker: If<ResMut<ProfileLoadTracker>>,
  mut next_state: ResMut<NextState<AppState>>,
) {
  // Track must be done loading
  if !tracker.task.is_finished() {
    return;
  }

  // Poll the completed load results
  let Some(result) = block_on(future::poll_once(&mut tracker.task)) else {
    // Should be impossible
    error!("Profile load task is missing when polling");
    panic!("Profile load task is none");
  };

  // Process the result
  let profile_data = match result {
    Ok(d) => d,
    Err(e) => {
      match e {
        ProfileLoadError::DeserializeFailed => {
          // File loaded but cannot be deserialized, cannot recover from this
          error!("Profile deserialize failed");
          panic!("Profile file corrupted");
        },
         ProfileLoadError::FileReadFailed => {
          // File cannot be accessed, cannot recover from this
          error!("Profile file read failed");
          panic!("Profile file unable to load");
        },
        ProfileLoadError::FileNotFound => {
          // File not found, assume new game
          info!("Profile file not found, assuming new game");
          ProfileData::default()
        },
      }
    },
  };

  // Remove the tracker
  commands.remove_resource::<ProfileLoadTracker>();

  // Create the profile context
  commands.insert_resource(ProfileContext(profile_data));

  // Proceed to main menu
  next_state.set(AppState::MainMenu);
}

fn poll_save_profile_requested(
  mut commands: Commands,
  mut reader: MessageReader<SaveProfileRequested>,
  profile_context: If<Res<ProfileContext>>,
  existing_tracker: Option<Res<ProfileSaveTracker>>,
) {
  // Must have save requests
  if reader.is_empty() {
    return;
  }

  // Clear reader, if multiple save requests are queued they will all be removed and we will load only once
  reader.clear();


  // Bail out if a load is already in progress
  if existing_tracker.is_some() {
    return;
  }

  // Write the profile context's data to the file in a separate thread
  let profile_data = (**profile_context).clone();
  let task: Task<Result<(), ProfileSaveError>> = IoTaskPool::get().spawn(async move {
    // Serialize the profile
    let contents = serde_json::to_string_pretty(&profile_data)
      .map_err(|_| ProfileSaveError::SerializeFailed)?;

    // Write the profile file
    std::fs::write(PROFILE_PATH, contents)
      .map_err(|_| ProfileSaveError::FileWriteFailed)?;

    // Save complete
    Ok(())
  });

  // Create the tracker
  commands.insert_resource(ProfileSaveTracker { task });
}

fn check_profile_saved(
  mut commands: Commands,
  mut writer: MessageWriter<SavedProfile>,
  mut tracker: If<ResMut<ProfileSaveTracker>>
) {
  // Tracker must be done loading
  if !tracker.task.is_finished() {
    return;
  }

  // Poll the completed save results
  let result = block_on(future::poll_once(&mut tracker.task))
    .expect("No saving profile result");

  // Remove the tracker
  commands.remove_resource::<ProfileSaveTracker>();

  // Signal save completed or failed
  writer.write(SavedProfile(result));
}