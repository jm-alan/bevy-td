mod mob_mortal;

use bevy::prelude::*;

use super::{
  damage::Damage,
  effect::{CurrentEffect, EffectResistance},
};

#[derive(Debug, Default, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct Mob {
  max_health: f32,
  current_health: f32,
  speed: f32,
  damage: Damage,
  damage_resistance: Damage,
  current_effect: CurrentEffect,
  effect_resistance: EffectResistance,
}
