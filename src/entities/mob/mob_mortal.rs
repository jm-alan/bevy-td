use super::Mob;
use crate::{
  entities::damage::Damage,
  traits::{damaging::Damaging, mortal::Mortal},
};

impl Mortal for Mob {
  fn take_damage<D>(&mut self, source: &D)
  where
    D: Damaging,
  {
    self.current_health -= self.process_damage(source.damage());
    self.current_effect = source.effect().current();
  }

  fn process_damage(&self, damage: &Damage) -> f32 {
    let mut total_damage: f32 = 0.0;
    total_damage += damage.standard - self.damage_resistance.standard;
    total_damage += damage.fire - self.damage_resistance.fire;
    total_damage += damage.ice - self.damage_resistance.ice;
    total_damage += damage.lightning - self.damage_resistance.lightning;
    total_damage += damage.acid - self.damage_resistance.acid;
    total_damage += damage.explosive - self.damage_resistance.explosive;
    total_damage
  }

  fn heal(&mut self) {}

  fn dead(&self) -> bool {
    self.current_health <= 0.0
  }
}
