pub mod f32_from_tower_size;
pub mod tower_base_stats_from_f32;
pub mod tower_base_stats_fusable;
pub mod tower_combat_stats_from_f32;
pub mod tower_combat_stats_fusable;
pub mod tower_components_from_f32;
pub mod tower_components_from_tower_kind;
pub mod tower_components_fusable;
pub mod tower_damaging;
pub mod tower_from_tower_kind;
pub mod tower_kind_default;
pub mod tower_kind_fusable;
pub mod tower_mortal;
pub mod tower_trigger_projectiles;
pub mod vec_tower_kind_recursive_fusable;

use bevy::prelude::*;

use super::{
  damage::Damage,
  effect::{CurrentEffect, Effect, EffectComponents},
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
      color,
    )
  }
}

#[derive(Debug, Default, Clone)]
pub struct TowerBaseStats {
  pub max_health: f32,
  pub healing_amount: f32,
}

#[derive(Debug, Default, Clone)]
pub struct TowerCombatStats {
  pub damage: f32,
  pub range: f32,
  pub fire_rate: f32,
  pub projectile_speed: f32,
}

#[derive(Debug, Default, Clone)]
pub struct TowerComponents {
  pub base_stats: TowerBaseStats,
  pub combat_stats: TowerCombatStats,
  pub effect_stats: EffectComponents,
}

#[derive(Debug, Reflect, Clone)]
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
