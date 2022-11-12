use super::Damage;
use crate::traits::fusable::Fusable;

impl Fusable for Damage {
  fn fuse(self, other: Self) -> Self {
    Self {
      standard: f32::max(self.standard, other.standard),
      fire: f32::max(self.fire, other.fire),
      ice: f32::max(self.ice, other.ice),
      lightning: f32::max(self.lightning, other.lightning),
      acid: f32::max(self.acid, other.acid),
      explosive: f32::max(self.explosive, other.explosive),
    }
  }
}
