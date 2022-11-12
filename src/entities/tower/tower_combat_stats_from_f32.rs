use super::TowerCombatStats;

impl From<[f32; 4]> for TowerCombatStats {
  #[inline(always)]
  fn from(parts: [f32; 4]) -> Self {
    Self {
      damage: parts[0],
      range: parts[1],
      fire_rate: parts[2],
      projectile_speed: parts[3],
    }
  }
}

impl From<&[f32]> for TowerCombatStats {
  #[inline(always)]
  fn from(parts: &[f32]) -> Self {
    Self {
      damage: parts[0],
      range: parts[1],
      fire_rate: parts[2],
      projectile_speed: parts[3],
    }
  }
}
