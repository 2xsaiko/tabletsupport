use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use crate::CONF_PATH;

pub fn on_rotation_update(rotation: &str) {
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