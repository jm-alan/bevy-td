use super::TowerSize;

use super::TowerSize::*;

impl From<TowerSize> for f32 {
  fn from(size: TowerSize) -> Self {
    match size {
      ExtraSmall => 0.25,
      Small => 0.5,
      Medium => 1.0,
      Large => 2.0,
      ExtraLarge => 4.0,
      Custom(dimension) => dimension,
    }
  }
}
