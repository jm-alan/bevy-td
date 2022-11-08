use crate::{
  entities::{damage::Damage, effect::Effect, tower::Tower},
  traits::damaging::Damaging,
};

impl Damaging for Tower {
  fn damage(&self) -> &Damage {
    &self.damage
  }
  fn effect(&self) -> &Effect {
    &self.damage_effect
  }
}
