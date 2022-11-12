use bevy::reflect::Reflect;

use crate::traits::{fusable::Fusable, possible::Possible};

use super::Chance;

impl<T> Fusable for Chance<T>
where
  T: Clone,
  T: Default,
  T: Possible,
  T: Reflect,
  T: Fusable,
{
  fn fuse(self, other: Self) -> Self {
    let wrapped = self.wrapped.clone().fuse(other.wrapped);
    Self {
      wrapped,
      probability: f32::max(self.probability, other.probability),
    }
  }
}
