use bevy::ecs::bundle::Bundle;

use crate::components::combat_health::CombatHealth;
use crate::components::combat_melee_attack::CombatMeleeAttack;

#[derive(Bundle)]
pub struct CombatBundle {
  pub health: CombatHealth,
  pub melee_attack: CombatMeleeAttack,
  // TODO abilities
}