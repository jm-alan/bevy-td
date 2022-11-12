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
    let components: TowerComponents;
    let damage: Damage;
    let damage_effect: Effect;

    match &kind {
      TowerKind::Hybrid(kinds) => {
        let fused_kinds = kinds.clone().recursive_fuse();
        let mut fused_damages = Damage::default();
        let mut fused_effects = Effect::default();
        let mut fused_components = TowerComponents::default();
        for fused_kind in fused_kinds.into_iter() {
          fused_damages = fused_damages.fuse(fused_kind.clone().into());
          fused_effects = fused_effects.fuse(fused_kind.clone().into());
          fused_components = fused_components.fuse(fused_kind.into());
        }
        damage = fused_damages;
        damage_effect = fused_effects;
        components = fused_components;
      }
      _ => {
        components = kind.clone().into();
        damage = kind.clone().into();
        damage_effect = kind.clone().into();
      }
    }

    let name: String = match &kind {
      TowerKind::Standard(_) => format!("standard-tower-{uuid}"),
      TowerKind::Fire(_) => format!("fire-tower-{uuid}"),
      TowerKind::Ice(_) => format!("ice-tower-{uuid}"),
      TowerKind::Lightning(_) => format!("lightning-tower-{uuid}"),
      TowerKind::Acid(_) => format!("acid-tower-{uuid}"),
      TowerKind::Explosive(_) => format!("explosive-tower-{uuid}"),
      TowerKind::Hybrid(kinds) => {
        let mut composed_name = "hybrid".to_string();
        for kind in kinds.iter() {
          match kind {
            TowerKind::Standard(_) => composed_name += "-standard",
            TowerKind::Fire(_) => composed_name += "-fire",
            TowerKind::Ice(_) => composed_name += "-ice",
            TowerKind::Lightning(_) => composed_name += "-lightning",
            TowerKind::Acid(_) => composed_name += "-acid",
            TowerKind::Explosive(_) => composed_name += "-explosive",
            TowerKind::Hybrid(_) => panic!("Failed to recursively fuse all nested hybrids before iterator consumption"),
          };
        }
        composed_name += "-tower";
        format!("{composed_name}-{uuid}")
      }
    };

    Self {
      name,
      damage,
      damage_effect,
      max_health: components.base_stats.max_health,
      current_health: components.base_stats.max_health,
      range: components.combat_stats.range,
      projectile_speed: components.combat_stats.projectile_speed,
      kind,
      current_effect: CurrentEffect::default(),
      damage_timer: Timer::from_seconds(
        1.0 / components.combat_stats.fire_rate,
        true,
      ),
      healing_timer: Timer::from_seconds(0.001, true),
      healing_amount: components.base_stats.healing_amount / 1000.0,
    }
  }
}
