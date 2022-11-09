mod f32_from_tower_size;
mod tower_damaging;
mod tower_from_tower_kind;
mod tower_mortal;
mod tower_trigger_projectiles;

use bevy::prelude::*;

use super::{
  damage::Damage,
  effect::{CurrentEffect, Effect},
  projectile::Projectile,
};
use crate::{generators::scene::Scene, traits::damaging::Damaging};

#[derive(Default, Reflect, Component, Clone)]
#[reflect(Component)]
pub struct Tower {
  name: String,
  kind: TowerKind,
  max_health: f32,
  current_health: f32,
  range: f32,
  projectile_speed: f32,
  damage: Damage,
  damage_timer: Timer,
  damage_effect: Effect,
  current_effect: CurrentEffect,
  healing_amount: f32,
  healing_timer: Timer,
}

impl Tower {
  pub fn spawn_projectile(&self) -> Projectile {
    Projectile::new(
      self.damage(),
      self.effect(),
      self.projectile_speed,
      self.range / self.projectile_speed,
    )
  }

  pub fn generate(
    kind: TowerKind,
    size: TowerSize,
    transform: Transform,
    color: Color,
  ) -> impl Fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>)
  {
    let tower: Tower = kind.into();
    let name: String = tower.name.clone();
    Scene::generate_shape(
      shape::Cube { size: size.into() },
      tower,
      name,
      transform,
      color.into(),
    )
  }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TowerComponents {
  max_health: f32,
  healing_amount: f32,
  base_damage: f32,
  effect_damage: f32,
  range: f32,
  fire_rate: f32,
  projectile_speed: f32,
}

impl From<[f32; 7]> for TowerComponents {
  fn from(parts: [f32; 7]) -> Self {
    Self {
      max_health: parts[0],
      healing_amount: parts[1],
      base_damage: parts[2],
      effect_damage: parts[3],
      range: parts[4],
      fire_rate: parts[5],
      projectile_speed: parts[6],
    }
  }
}

#[derive(Debug, Clone, Reflect)]
pub enum TowerKind {
  Standard(TowerComponents),
  Fire(TowerComponents),
  Ice(TowerComponents),
  Lightning(TowerComponents),
  Acid(TowerComponents),
  Explosive(TowerComponents),
  Hybrid(Vec<TowerKind>),
}

#[derive(Debug, Default, Copy, Clone, Reflect)]
pub enum TowerSize {
  #[default]
  Medium,
  ExtraSmall,
  Small,
  Large,
  ExtraLarge,
  Custom(f32),
}

impl Default for TowerKind {
  fn default() -> Self {
    TowerKind::Standard([0.0; 7].into())
  }
}
