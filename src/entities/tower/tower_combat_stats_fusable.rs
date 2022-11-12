use crate::traits::fusable::Fusable;

use super::TowerCombatStats;

impl Fusable for TowerCombatStats {
  fn fuse(self, other: Self) -> Self {
    Self {
      damage: f32::max(self.damage, other.damage),
      range: f32::max(self.range, other.range),
      fire_rate: f32::max(self.fire_rate, other.fire_rate),
      projectile_speed: f32::max(self.projectile_speed, other.projectile_speed),
    }
  }
}
