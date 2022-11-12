use bevy::time::Timer;
use uuid::Uuid;

use super::{Tower, TowerComponents, TowerKind};
use crate::{
  entities::{
    damage::Damage,
    effect::{CurrentEffect, Effect},
  },
  traits::fusable::{Fusable, RecursiveFusable},
};

impl From<TowerKind> for Tower {
  #[inline(always)]
  fn from(kind: TowerKind) -> Self {
    let uuid: String = Uuid::new_v4().to_string();
    let name: String;
    let mut components: TowerComponents = kind.clone().into();
    let mut damage: Damage = kind.clone().into();
    let mut damage_effect: Effect = kind.clone().into();

    match &kind {
      TowerKind::Standard(_) => {
        name = format!("standard-tower-{uuid}");
      }
      TowerKind::Fire(_) => {
        name = format!("fire-tower-{uuid}");
      }
      TowerKind::Ice(_) => {
        name = format!("ice-tower-{uuid}");
      }
      TowerKind::Lightning(_) => {
        name = format!("lightning-tower-{uuid}");
      }
      TowerKind::Acid(_) => {
        name = format!("acid-tower-{uuid}");
      }
      TowerKind::Explosive(_) => {
        name = format!("explosive-tower-{uuid}");
      }
      TowerKind::Hybrid(kinds) => {
        name = format!("explosive-tower-{uuid}");
        let fused_kinds = kinds
          .clone()
          .recursive_fuse();
        let mut fused_damages = Damage::default();
        let mut fused_effects = Effect::default();
        let mut fused_components = TowerComponents::default();
        for el in fused_kinds.into_iter() {
          fused_damages = fused_damages.fuse(el.clone().into());
          fused_effects = fused_effects.fuse(el.clone().into());
          fused_components = fused_components.fuse(el.into());
        }
        damage = fused_damages;
        damage_effect = fused_effects;
        components = fused_components;
      }
    };

    Self {
      name,
      damage,
      damage_effect,
      max_health: components
        .base_stats
        .max_health,
      current_health: components
        .base_stats
        .max_health,
      range: components
        .combat_stats
        .range,
      projectile_speed: components
        .combat_stats
        .projectile_speed,
      kind,
      current_effect: CurrentEffect::default(),
      damage_timer: Timer::from_seconds(
        1.0
          / components
            .combat_stats
            .fire_rate,
        true,
      ),
      healing_timer: Timer::from_seconds(0.001, true),
      healing_amount: components
        .base_stats
        .healing_amount
        / 1000.0,
    }
  }
}
