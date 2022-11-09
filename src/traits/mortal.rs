use crate::entities::damage::Damage;

use super::damaging::Damaging;

pub trait Mortal {
  fn take_damage<D>(&mut self, source: &D)
  where
    D: Damaging;
  fn process_damage(&self, damage: &Damage) -> f32;
  fn heal(&mut self);
  fn dead(&self) -> bool;
}
