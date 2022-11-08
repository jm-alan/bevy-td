mod entities;
mod generators;
mod traits;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use entities::{
  damage::Damage, effect::Effect, mob::Mob, projectile::Projectile,
  tower::Tower,
};
use generators::{camera::spawn_camera, light::spawn_light, scene::Scene};

pub const WINDOW_WIDTH: f32 = 1600.0;
pub const WINDOW_HEIGHT: f32 = 900.0;

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(WindowDescriptor {
      width: WINDOW_WIDTH,
      height: WINDOW_HEIGHT,
      title: "Bevy TD Demo".to_string(),
      resizable: false,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::new())
    .register_type::<Tower>()
    .register_type::<Projectile>()
    .register_type::<Mob>()
    .add_startup_system(Scene::plane_generator(Color::rgb(0.5, 0.1, 0.1), 5.0))
    .add_startup_system(Scene::shape_generator(
      shape::Cube { size: 1.0 },
      Tower::new(
        100.0,
        0.0,
        10.0,
        1.0,
        1.0,
        Damage {
          standard: 1.0,
          ..Default::default()
        },
        Effect::default(),
      ),
      "Tower".to_string(),
      Transform::from_xyz(0.0, 0.5, 0.0),
      Color::rgb(0.9, 0.3, 0.3),
    ))
    .add_startup_system(spawn_light)
    .add_startup_system(spawn_camera)
    .add_system(Tower::trigger_projectiles)
    .add_system(Projectile::trigger_despawn)
    .run();
}
