use super::Damage;
use crate::{entities::tower::TowerKind, traits::fusable::Fusable};

impl From<TowerKind> for Damage {
  fn from(kind: TowerKind) -> Self {
    match kind {
      TowerKind::Standard(components) => Self {
        standard: components.combat_stats.damage,
        ..Default::default()
      },
      TowerKind::Fire(components) => Self {
        fire: components.combat_stats.damage,
        ..Default::default()
      },
      TowerKind::Ice(components) => Self {
        ice: components.combat_stats.damage,
        ..Default::default()
      },
      TowerKind::Lightning(components) => Self {
        lightning: components.combat_stats.damage,
        ..Default::default()
      },
      TowerKind::Acid(components) => Self {
        acid: components.combat_stats.damage,
        ..Default::default()
      },
      TowerKind::Explosive(components) => Self {
        explosive: components.combat_stats.damage,
        ..Default::default()
      },
      TowerKind::Hybrid(kinds) => {
        let mut damage = Self::default();
        for kind in kinds.into_iter() {
          damage = damage.fuse(kind.into());
        }
        damage
      }
    }
  }
}
