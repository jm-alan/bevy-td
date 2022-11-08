use crate::{
  entities::{damage::Damage, effect::Effect, projectile::Projectile},
  traits::damaging::Damaging,
};

impl Damaging for Projectile {
  fn damage(&self) -> &Damage {
    &self.damage
  }
  fn effect(&self) -> &Effect {
    &self.effect
  }
}
