use crate::traits::fusable::Fusable;

use super::TowerComponents;

impl Fusable for TowerComponents {
  fn fuse(self, other: Self) -> Self {
    Self {
      base_stats: self
        .base_stats
        .fuse(other.base_stats),
      combat_stats: self
        .combat_stats
        .fuse(other.combat_stats),
      effect_stats: self
        .effect_stats
        .fuse(other.effect_stats),
    }
  }
}
