use super::EffectComponents;
use crate::traits::fusable::Fusable;

impl Fusable for EffectComponents {
  fn fuse(self, other: Self) -> Self {
    Self {
      damage: self.damage + other.damage,
      duration: self.duration + other.duration,
      probability: f32::max(self.probability, other.probability),
    }
  }
}
