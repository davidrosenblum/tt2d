use bevy::ecs::component::Component;

use crate::models::cog_department_code::CogDepartmentCode;

#[derive(Component)]
pub struct Cog {
  pub name: String,
  pub department_code: CogDepartmentCode,
}