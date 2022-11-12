use super::Effect;
use crate::traits::fusable::Fusable;

impl Fusable for Effect {
  fn fuse(self, other: Self) -> Self {
    Self {
      burn: self.burn.fuse(other.burn),
      freeze: self.freeze.fuse(other.freeze),
      shock: self.shock.fuse(other.shock),
      corrode: self.corrode.fuse(other.corrode),
    }
  }
}
