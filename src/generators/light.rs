use bevy::prelude::*;

pub fn spawn_light(mut commands: Commands) {
  commands.spawn_bundle(PointLightBundle {
    point_light: PointLight {
      intensity: 2000.0,
      shadows_enabled: true,
      ..Default::default()
    },
    transform: Transform::from_xyz(5.0, 10.0, 3.0),
    ..Default::default()
  });
}

pub struct Light;
