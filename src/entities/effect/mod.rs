pub mod effect_components_possible;

use bevy::prelude::*;

use super::chance::Chance;

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub struct EffectComponents {
  pub probability: f32,
  pub duration: f32,
  pub damage: f32,
}

impl EffectComponents {
  pub fn running(&self) -> RunningEffect {
    RunningEffect::new(self.damage, self.duration)
  }
}

#[derive(Debug, Default, Clone, Reflect)]
pub struct RunningEffect {
  pub damage: f32,
  pub timer: Timer,
}

impl RunningEffect {
  pub fn new(damage: f32, duration: f32) -> Self {
    Self {
      damage,
      timer: Timer::from_seconds(duration, false),
    }
  }
}

#[derive(Debug, Default, Clone, Reflect)]
pub struct CurrentEffect {
  pub burn: RunningEffect,
  pub freeze: RunningEffect,
  pub shock: RunningEffect,
  pub corrode: RunningEffect,
}

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub struct Effect {
  pub burn: Chance<EffectComponents>,
  pub freeze: Chance<EffectComponents>,
  pub shock: Chance<EffectComponents>,
  pub corrode: Chance<EffectComponents>,
}

impl Effect {
  pub fn current(&self) -> CurrentEffect {
    CurrentEffect {
      burn: self.burn.maybe().running(),
      freeze: self.freeze.maybe().running(),
      shock: self.shock.maybe().running(),
      corrode: self.corrode.maybe().running(),
    }
  }
}
