#![feature(never_type)]

use std::sync::{Arc, RwLock};

mod dispatcher;
mod rotation_handler;
mod sensorproxy;
mod tray;

const CONF_PATH: &str = "./conf.d/rotation";

fn main() {
  let state = Arc::new(RwLock::new(SystemState::default()));
  let threads = vec![
    tray::start(state.clone()),
    rotation_handler::start(state.clone()),
  ];
  threads.into_iter().for_each(|t| { t.join().unwrap(); });
}

#[derive(Default, Debug, Clone)]
pub struct SystemState {
  lock_orientation: bool,
  orientation: Orientation,
  actual_orientation: Orientation,
}

impl SystemState {
  pub fn set_actual_orientation(&mut self, o: Orientation) {
    self.actual_orientation = o;
    self.on_orientation_changed();
  }

  pub fn actual_orientation(&self) -> Orientation { self.actual_orientation }

  pub fn set_orientation(&mut self, o: Orientation) {
    self.lock_orientation = true;
    self.orientation = o;
    self.update_listeners();
  }

  pub fn set_lock_orientation(&mut self, b: bool) {
    self.lock_orientation = b;
    if !b { self.on_orientation_changed(); }
  }

  pub fn toggle_lock_orientation(&mut self) {
    self.set_lock_orientation(!self.lock_orientation());
  }

  pub fn lock_orientation(&self) -> bool { self.lock_orientation }

  fn on_orientation_changed(&mut self) {
    if !self.lock_orientation && self.orientation != self.actual_orientation {
      self.orientation = self.actual_orientation;
      self.update_listeners();
    }
  }

  pub fn update_listeners(&self) {
    dispatcher::on_rotation_update(self.orientation.as_str());
  }

  // TODO save state?
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
  Normal,
  LeftUp,
  RightUp,
  BottomUp,
}

impl Orientation {
  fn from_str(s: &str) -> Orientation {
    match s {
      "normal" => Orientation::Normal,
      "left-up" => Orientation::LeftUp,
      "right-up" => Orientation::RightUp,
      "bottom-up" => Orientation::BottomUp,
      _ => unreachable!(),
    }
  }

  fn as_str(&self) -> &'static str {
    match self {
      Orientation::Normal => "normal",
      Orientation::LeftUp => "left-up",
      Orientation::RightUp => "right-up",
      Orientation::BottomUp => "bottom-up",
    }
  }
}

impl Default for Orientation {
  fn default() -> Self { Orientation::Normal }
}