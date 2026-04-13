use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};

use crate::models::facing_code::FacingCode;

#[derive(Component, Deref, DerefMut)]
pub struct UnitFacing(pub FacingCode);