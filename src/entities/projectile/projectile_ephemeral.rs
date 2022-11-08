use bevy::time::Time;

use crate::{entities::projectile::Projectile, traits::ephemeral::Ephemeral};

impl Ephemeral for Projectile {
  fn age(&mut self, time: &Time) {
    self.lifetime.age(time)
  }

  fn finished(&self) -> bool {
    self.lifetime.finished()
  }

  fn just_finished(&self) -> bool {
    self.lifetime.just_finished()
  }
}
