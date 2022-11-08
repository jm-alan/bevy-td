use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub struct Damage {
  pub standard: f32,
  pub fire: f32,
  pub ice: f32,
  pub lightning: f32,
  pub acid: f32,
  pub explosive: f32,
}
