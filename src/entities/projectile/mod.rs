mod projectile_damaging;
mod projectile_ephemeral;
mod projectile_trigger_despawn;

use bevy::prelude::*;

use super::{damage::Damage, effect::Effect, lifetime::Lifetime};

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Projectile {
  damage: Damage,
  effect: Effect,
  speed: f32,
  lifetime: Lifetime,
}

impl Projectile {
  pub fn new(
    damage: &Damage,
    effect: &Effect,
    speed: f32,
    duration: f32,
  ) -> Self {
    Self {
      damage: damage.to_owned(),
      effect: effect.to_owned(),
      speed,
      lifetime: Lifetime::new(duration),
    }
  }
}
