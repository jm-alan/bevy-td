use crate::entities::effect::EffectComponents;

use super::Chance;

impl From<EffectComponents> for Chance<EffectComponents> {
  #[inline(always)]
  fn from(components: EffectComponents) -> Self {
    Self::new(components, components.probability)
  }
}
