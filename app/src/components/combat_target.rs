use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::prelude::{Deref, DerefMut};

#[derive(Component, Deref, DerefMut)]
pub struct CombatTarget(pub Entity);