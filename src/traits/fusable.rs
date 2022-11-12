pub trait Fusable {
  fn fuse(self, other: Self) -> Self;
}

pub trait RecursiveFusable: Fusable + Sized {
  fn recursive_fuse(self) -> Self;
}
