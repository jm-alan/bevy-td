pub trait Fusable: Clone {
  fn fuse(&self, other: &Self) -> Self;
  fn absorb(&mut self, other: &Self);
}
