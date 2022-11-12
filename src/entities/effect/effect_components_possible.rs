use super::EffectComponents;
use crate::traits::possible::Possible;

impl Possible for EffectComponents {
  fn success(&self) -> Self {
    self.clone()
  }

  fn failure(&self) -> Self {
    Self::default()
  }
}
