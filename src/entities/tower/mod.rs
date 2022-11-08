mod tower_damaging;
mod tower_mortal;
mod tower_trigger_projectiles;

use bevy::prelude::*;

use super::{
  damage::Damage,
  effect::{CurrentEffect, Effect},
  projectile::Projectile,
};
use crate::traits::damaging::Damaging;

#[derive(Default, Reflect, Component, Clone)]
#[reflect(Component)]
pub struct Tower {
  max_health: f32,
  current_health: f32,
  range: f32,
  projectile_speed: f32,
  damage: Damage,
  damage_timer: Timer,
  damage_effect: Effect,
  current_effect: CurrentEffect,
  healing_amount: f32,
  healing_timer: Timer,
}

impl Tower {
  pub fn new(
    max_health: f32,
    healing_amount: f32,
    range: f32,
    fire_rate: f32,
    projectile_speed: f32,
    damage: Damage,
    damage_effect: Effect,
  ) -> Self {
    Self {
      max_health,
      current_health: max_health,
      damage,
      range,
      projectile_speed,
      damage_effect,
      current_effect: CurrentEffect::default(),
      damage_timer: Timer::from_seconds(1.0 / fire_rate, true),
      healing_timer: Timer::from_seconds(0.001, true),
      healing_amount: healing_amount / 1000.0,
    }
  }

  pub fn spawn_projectile(&self) -> Projectile {
    Projectile::new(
      self.damage(),
      self.effect(),
      self.projectile_speed,
      self.range / self.projectile_speed,
    )
  }
}
