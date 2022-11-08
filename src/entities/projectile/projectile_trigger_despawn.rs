use bevy::prelude::*;

use super::Projectile;
use crate::traits::ephemeral::Ephemeral;

impl Projectile {
  pub fn trigger_despawn(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Projectile)>,
    time: Res<Time>,
  ) {
    for (entity, mut projectile) in &mut projectiles {
      projectile.age(&time);
      if projectile.just_finished() {
        commands.entity(entity).despawn_recursive()
      }
    }
  }
}
