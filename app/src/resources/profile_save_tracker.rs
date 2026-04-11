use bevy::ecs::resource::Resource;
use bevy::tasks::Task;

use crate::errors::profile_save_error::ProfileSaveError;

#[derive(Resource)]
pub struct ProfileSaveTracker {
  pub task: Task<Result<(), ProfileSaveError>>,
}