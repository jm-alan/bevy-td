use super::{TowerComponents, TowerKind};
use crate::traits::fusable::Fusable;

impl From<TowerKind> for TowerComponents {
  fn from(kind: TowerKind) -> Self {
    match kind {
      TowerKind::Standard(components) => components,
      TowerKind::Fire(components) => components,
      TowerKind::Ice(components) => components,
      TowerKind::Lightning(components) => components,
      TowerKind::Acid(components) => components,
      TowerKind::Explosive(components) => components,
      TowerKind::Hybrid(kinds) => {
        let mut components = TowerComponents::default();
        for kind in kinds.into_iter() {
          components = components.fuse(kind.into());
        }
        components
      }
    }
  }
}
