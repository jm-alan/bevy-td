pub mod chance_effect_component_from_effect_component;
pub mod chance_effect_component_from_f32;
pub mod chance_fusable;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::traits::possible::Possible;

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct Chance<T>
where
  T: Clone,
  T: Default,
  T: Possible,
  T: Reflect,
{
  wrapped: T,
  probability: f32,
}

impl<T> Chance<T>
where
  T: Clone,
  T: Default,
  T: Possible,
  T: Reflect,
{
  pub fn new(wrapped: T, probability: f32) -> Self {
    Self {
      wrapped,
      probability,
    }
  }

  pub fn maybe(&self) -> T {
    if thread_rng().gen_range(0.0..100.0) < self.probability {
      self.wrapped.success()
    } else {
      self.wrapped.failure()
    }
  }
}
