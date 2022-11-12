use super::TowerBaseStats;

impl From<[f32; 2]> for TowerBaseStats {
  #[inline(always)]
  fn from(parts: [f32; 2]) -> Self {
    Self {
      max_health: parts[0],
      healing_amount: parts[1],
    }
  }
}

impl From<&[f32]> for TowerBaseStats {
  #[inline(always)]
  fn from(parts: &[f32]) -> Self {
    Self {
      max_health: parts[0],
      healing_amount: parts[1],
    }
  }
}
