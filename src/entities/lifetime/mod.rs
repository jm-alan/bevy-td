use bevy::{prelude::*, time::Timer};

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
  timer: Timer,
}

impl Lifetime {
  pub fn new(duration: f32) -> Self {
    Self {
      timer: Timer::from_seconds(duration, false),
    }
  }

  pub fn age(&mut self, time: &Time) {
    self.timer.tick(time.delta());
  }

  pub fn finished(&self) -> bool {
    self.timer.finished()
  }

  pub fn just_finished(&self) -> bool {
    self.timer.just_finished()
  }
}
