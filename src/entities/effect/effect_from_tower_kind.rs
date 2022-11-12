use super::Effect;
use crate::{entities::tower::TowerKind, traits::fusable::Fusable};

impl From<TowerKind> for Effect {
  fn from(kind: TowerKind) -> Self {
    match kind {
      TowerKind::Fire(components) => Self {
        burn: components.effect_stats.into(),
        ..Default::default()
      },
      TowerKind::Ice(components) => Self {
        freeze: components.effect_stats.into(),
        ..Default::default()
      },
      TowerKind::Lightning(components) => Self {
        shock: components.effect_stats.into(),
        ..Default::default()
      },
      TowerKind::Acid(components) => Self {
        corrode: components.effect_stats.into(),
        ..Default::default()
      },
      TowerKind::Hybrid(kinds) => {
        let mut effect = Self::default();
        for kind in kinds.into_iter() {
          effect = effect.fuse(kind.into());
        }
        effect
      }
      _ => Self::default(),
    }
  }
}
