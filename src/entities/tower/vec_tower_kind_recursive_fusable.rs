use super::TowerKind;
use crate::traits::fusable::{Fusable, RecursiveFusable};

impl Fusable for Vec<TowerKind> {
  fn fuse(self, other: Self) -> Self {
    [self, other].concat()
  }
}

impl RecursiveFusable for Vec<TowerKind> {
  fn recursive_fuse(self) -> Self {
    let mut standard: Vec<TowerKind> = vec![];
    let mut fire: Vec<TowerKind> = vec![];
    let mut ice: Vec<TowerKind> = vec![];
    let mut lightning: Vec<TowerKind> = vec![];
    let mut acid: Vec<TowerKind> = vec![];
    let mut explosive: Vec<TowerKind> = vec![];
    let mut hybrid: Vec<TowerKind> = vec![];
    for el in self.into_iter() {
      match el {
        TowerKind::Standard(_) => standard.push(el),
        TowerKind::Fire(_) => fire.push(el),
        TowerKind::Ice(_) => ice.push(el),
        TowerKind::Lightning(_) => lightning.push(el),
        TowerKind::Acid(_) => acid.push(el),
        TowerKind::Explosive(_) => explosive.push(el),
        TowerKind::Hybrid(_) => hybrid.push(el),
      };
    }
    let mut combined: Vec<TowerKind> = vec![];
    if let Option::Some(fused) = standard
      .into_iter()
      .reduce(move |acc, next| acc.fuse(next))
    {
      combined.push(fused);
    };
    if let Option::Some(fused) = fire
      .into_iter()
      .reduce(move |acc, next| acc.fuse(next))
    {
      combined.push(fused);
    };
    if let Option::Some(fused) = ice
      .into_iter()
      .reduce(move |acc, next| acc.fuse(next))
    {
      combined.push(fused);
    };
    if let Option::Some(fused) = lightning
      .into_iter()
      .reduce(move |acc, next| acc.fuse(next))
    {
      combined.push(fused);
    };
    if let Option::Some(fused) = acid
      .into_iter()
      .reduce(move |acc, next| acc.fuse(next))
    {
      combined.push(fused);
    };
    if let Option::Some(fused) = explosive
      .into_iter()
      .reduce(move |acc, next| acc.fuse(next))
    {
      combined.push(fused);
    };
    if let Option::Some(fused) = hybrid
      .into_iter()
      .reduce(move |acc, next| acc.fuse(next))
    {
      combined.push(fused);
    };
    combined
  }
}
