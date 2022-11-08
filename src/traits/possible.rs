pub trait Possible {
  fn success(&self) -> Self;
  fn failure(&self) -> Self;
}
