use bevy::time::Time;

pub trait Ephemeral {
  fn age(&mut self, time: &Time);
  fn finished(&self) -> bool;
  fn just_finished(&self) -> bool;
}
