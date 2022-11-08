mod mob_mortal;

use bevy::prelude::*;

use super::{damage::Damage, effect::CurrentEffect};

#[derive(Debug, Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Mob {
  max_health: f32,
  current_health: f32,
  speed: f32,
  damage: Damage,
  current_effect: CurrentEffect,
}
