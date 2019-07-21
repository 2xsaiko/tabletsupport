use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;

use dbus::*;
use dbus::arg::{RefArg, Variant};

use crate::{Orientation, SystemState};
use crate::sensorproxy::OrgFreedesktopDBusPropertiesPropertiesChanged as PropertiesChanged;

pub fn start(state: Arc<RwLock<SystemState>>) -> JoinHandle<()> {
  thread::spawn(|| run(state))
}

pub fn run(state: Arc<RwLock<SystemState>>) {
  let c = Connection::get_private(BusType::System).unwrap();
  let p = c.with_path("net.hadess.SensorProxy", "/net/hadess/SensorProxy", 5000);
  use crate::sensorproxy::NetHadessSensorProxy;
  let has_accel = p.get_has_accelerometer().unwrap();
  println!("Has acceleration sensor: {}", has_accel);
  if has_accel {
    p.claim_accelerometer().unwrap();
    c.add_match(&PropertiesChanged::match_str(Some(&"net.hadess.SensorProxy".into()), None)).unwrap();

    // call scripts with initial value before listening to updates
    state.write().unwrap().set_actual_orientation(Orientation::from_str(&p.get_accelerometer_orientation().unwrap()));

    loop {
      for msg in c.incoming(1000) {
        if let Some(a) = PropertiesChanged::from_message(&msg) as Option<PropertiesChanged> {
          for (k, v) in a.changed_properties.iter() {
            let v = v as &Variant<Box<dyn RefArg>>;
            match k.as_str() {
              "AccelerometerOrientation" => {
                if let Some(ao) = v.as_str() {
                  state.write().unwrap().set_actual_orientation(Orientation::from_str(ao));
                }
              }
              _ => {}
            }
          }
        }
      }
    }
  }
}