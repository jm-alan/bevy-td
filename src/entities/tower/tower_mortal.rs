use super::Tower;
use crate::{
  entities::damage::Damage,
  traits::{damaging::Damaging, mortal::Mortal},
};

impl Mortal for Tower {
  fn take_damage<D>(&mut self, source: &D)
  where
    D: Damaging,
  {
    self.current_health -= self.process_damage(source.damage());
    self.current_effect = source.effect().current();
  }

  fn process_damage(&self, damage: &Damage) -> f32 {
    todo!()
  }

  fn heal(&mut self) {
    if self.healing_amount == 0.0 {
      return;
    }
    if self.current_health < self.max_health {
      if self.healing_amount > (self.max_health - self.current_health) {
        self.current_health = self.max_health;
      } else {
        self.current_health += self.healing_amount;
      }
    }
  }

  fn dead(&self) -> bool {
    self.current_health == 0.0
  }
}
