use super::TowerKind;

impl Default for TowerKind {
  fn default() -> Self {
    TowerKind::Standard([0.0; 9].into())
  }
}
