use crate::entities::{damage::Damage, effect::Effect};

pub trait Damaging {
  fn damage(&self) -> &Damage;
  fn effect(&self) -> &Effect;
}
