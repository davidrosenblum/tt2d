use bevy::ecs::resource::Resource;
use bevy::prelude::{Deref, DerefMut};

use crate::models::profile_data::ProfileData;

#[derive(Resource, Deref, DerefMut)]
pub struct ProfileContext(pub ProfileData);