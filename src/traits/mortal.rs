use crate::entities::damage::Damage;

use super::damaging::Damaging;

pub trait Mortal {
  fn take_damage<D>(&mut self, source: &D)
  where
    D: Damaging;
  fn apply_resistances(&self, damage: &Damage) -> Damage;
  fn heal(&mut self);
  fn dead(&self) -> bool;
}
