use super::TowerKind;
use crate::traits::fusable::{Fusable, RecursiveFusable};

impl Fusable for TowerKind {
  fn fuse(self, other: Self) -> Self {
    match (self, other) {
      (TowerKind::Hybrid(mut lhs), TowerKind::Hybrid(ref mut rhs)) => {
        lhs.append(rhs);
        TowerKind::Hybrid(lhs.recursive_fuse())
      }
      (TowerKind::Hybrid(mut lhs), rhs) => {
        lhs.push(rhs);
        TowerKind::Hybrid(lhs.recursive_fuse())
      }
      (lhs, TowerKind::Hybrid(mut rhs)) => {
        rhs.push(lhs);
        TowerKind::Hybrid(rhs.recursive_fuse())
      }
      (lhs, rhs) => TowerKind::Hybrid(vec![lhs, rhs]),
    }
  }
}
