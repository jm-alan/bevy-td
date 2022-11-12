use super::TowerBaseStats;
use crate::traits::fusable::Fusable;

impl Fusable for TowerBaseStats {
  fn fuse(self, other: Self) -> Self {
    Self {
      max_health: f32::max(self.max_health, other.max_health),
      healing_amount: f32::max(self.healing_amount, other.healing_amount),
    }
  }
}
