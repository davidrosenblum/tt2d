use bevy::ecs::resource::Resource;
use bevy::tasks::Task;

use crate::errors::profile_load_error::ProfileLoadError;
use crate::models::profile_data::ProfileData;

#[derive(Resource)]
pub struct ProfileLoadTracker {
  pub task: Task<Result<ProfileData, ProfileLoadError>>,
}