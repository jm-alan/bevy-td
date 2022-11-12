use super::Chance;
use crate::entities::effect::EffectComponents;

impl From<[f32; 3]> for Chance<EffectComponents> {
  #[inline(always)]
  fn from(parts: [f32; 3]) -> Self {
    Self {
      wrapped: parts.into(),
      probability: parts[0],
    }
  }
}
