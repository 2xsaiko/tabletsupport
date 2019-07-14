use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use dbus::*;
use dbus::arg::{RefArg, Variant};

use sensorproxy::OrgFreedesktopDBusPropertiesPropertiesChanged as PropertiesChanged;

mod sensorproxy;

const CONF_PATH: &str = "./conf.d/rotation";

fn main() {
  let c = Connection::get_private(BusType::System).unwrap();
  let p = c.with_path("net.hadess.SensorProxy", "/net/hadess/SensorProxy", 5000);
  use sensorproxy::NetHadessSensorProxy;
  let has_accel = p.get_has_accelerometer().unwrap();
  println!("Has acceleration sensor: {}", has_accel);
  if has_accel {
    p.claim_accelerometer().unwrap();
    c.add_match(&PropertiesChanged::match_str(Some(&"net.hadess.SensorProxy".into()), None)).unwrap();

    // call scripts with initial value before listening to updates
    on_rotation_update(&p.get_accelerometer_orientation().unwrap());

    loop {
      for msg in c.incoming(1000) {
        if let Some(a) = PropertiesChanged::from_message(&msg) as Option<PropertiesChanged> {
          for (k, v) in a.changed_properties.iter() {
            let v = v as &Variant<Box<dyn RefArg>>;
            match k.as_str() {
              "AccelerometerOrientation" => {
                if let Some(ao) = v.as_str() {
                  on_rotation_update(ao);
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

fn on_rotation_update(rotation: &str) {
  for rd in fs::read_dir(CONF_PATH) {
    rd.into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.path().is_file() && e.metadata().unwrap().permissions().mode() & 0o111 != 0)
      .for_each(|e| {
        Command::new(e.path())
          .arg(rotation)
          .spawn().expect(&format!("Failed to exec {}", e.path().to_string_lossy()));
      });
  }
}