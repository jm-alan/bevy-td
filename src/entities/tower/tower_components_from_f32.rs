use super::TowerComponents;

impl From<[f32; 9]> for TowerComponents {
  #[inline(always)]
  fn from(parts: [f32; 9]) -> Self {
    Self {
      base_stats: parts[0..1].into(),
      combat_stats: parts[2..5].into(),
      effect_stats: parts[6..8].try_into().unwrap(),
    }
  }
}

impl From<&[f32]> for TowerComponents {
  #[inline(always)]
  fn from(parts: &[f32]) -> Self {
    Self {
      base_stats: parts[0..1].into(),
      combat_stats: parts[2..5].into(),
      effect_stats: parts[6..8].try_into().unwrap(),
    }
  }
}
