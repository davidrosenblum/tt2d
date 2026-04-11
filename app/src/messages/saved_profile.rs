use bevy::ecs::message::Message;

use crate::errors::profile_save_error::ProfileSaveError;

#[derive(Message)]
pub struct SavedProfile(pub Result<(), ProfileSaveError>);