use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::prelude::{Deref, DerefMut};

#[derive(Component, Deref, DerefMut)]
pub struct CogSpawnPoint(pub Vec2);