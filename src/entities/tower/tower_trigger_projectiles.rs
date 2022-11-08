use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use super::Tower;
use crate::traits::mortal::Mortal;

impl Tower {
  pub fn trigger_projectiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
  ) {
    let mut rng = thread_rng();
    for mut tower in &mut towers {
      tower.damage_timer.tick(time.delta());
      if tower.damage_timer.just_finished() {
        let spawn_transform = Transform::from_xyz(
          rng.gen_range(0.0..2.0),
          rng.gen_range(0.0..2.0),
          rng.gen_range(0.0..2.0),
        )
        .with_rotation(Quat::from_rotation_y(-PI / 2.0));
        commands
          .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.9, 0.4, 0.4).into()),
            transform: spawn_transform,
            ..Default::default()
          })
          .insert(tower.spawn_projectile())
          .insert(Name::new("Pew"));
      }
      tower.healing_timer.tick(time.delta());
      if tower.healing_timer.just_finished() {
        tower.heal();
      }
    }
  }
}
