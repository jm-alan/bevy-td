use bevy::time::Timer;
use uuid::Uuid;

use super::{
  Tower, TowerComponents,
  TowerKind::{self, *},
};
use crate::entities::{
  damage::Damage,
  effect::{CurrentEffect, Effect},
};

impl From<TowerKind> for Tower {
  fn from(kind: TowerKind) -> Self {
    let uuid = Uuid::new_v4().to_string();
    let components: TowerComponents;
    let name: String;
    let damage: Damage;
    let damage_effect: Effect;

    match kind {
      Standard(comps) => {
        name = format!("standard-tower-{uuid}").to_string();
        components = comps;
      }
      Fire(comps) => {
        name = format!("fire-tower-{uuid}").to_string();
        components = comps;
      }
      Ice(comps) => {
        name = format!("ice-tower-{uuid}").to_string();
        components = comps;
      }
      Lightning(comps) => {
        name = format!("lightning-tower-{uuid}").to_string();
        components = comps;
      }
      Acid(comps) => {
        name = format!("acid-tower-{uuid}").to_string();
        components = comps;
      }
      Explosive(comps) => {
        name = format!("explosive-tower-{uuid}").to_string();
        components = comps;
      }
      Hybrid(kinds) => todo!(),
    };

    Self {
      name,
      damage,
      damage_effect,
      max_health: components.max_health,
      current_health: components.max_health,
      range: components.range,
      projectile_speed: components.projectile_speed,
      kind: kind.clone(),
      current_effect: CurrentEffect::default(),
      damage_timer: Timer::from_seconds(1.0 / components.fire_rate, true),
      healing_timer: Timer::from_seconds(0.001, true),
      healing_amount: components.healing_amount / 1000.0,
    }
  }
}
