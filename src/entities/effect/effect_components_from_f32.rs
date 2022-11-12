use super::EffectComponents;

impl From<[f32; 3]> for EffectComponents {
  #[inline(always)]
  fn from(parts: [f32; 3]) -> Self {
    Self {
      probability: parts[0],
      duration: parts[1],
      damage: parts[2],
    }
  }
}

impl From<&[f32]> for EffectComponents {
  #[inline(always)]
  fn from(parts: &[f32]) -> Self {
    Self {
      probability: parts[0],
      duration: parts[1],
      damage: parts[2],
    }
  }
}
