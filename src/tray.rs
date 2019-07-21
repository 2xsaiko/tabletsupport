use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;

use ksni::tray::{Category, Icon, ToolTip};

use crate::SystemState;

pub fn start(state: Arc<RwLock<SystemState>>) -> JoinHandle<()> {
  thread::spawn(|| run(state))
}

pub fn run(state: Arc<RwLock<SystemState>>) {
  struct Tray {
    state: Arc<RwLock<SystemState>>
  };

  // TODO this doesn't have any sort of feedback to the user right now, but that's a limitation of ksni

  impl ksni::Tray for Tray {
    type Err = !;

    fn activate(&self, _x: i32, _y: i32) -> Result<(), Self::Err> {
      let mut s = self.state.write().unwrap();
      s.toggle_lock_orientation();
      Ok(())
    }

    fn tray_properties() -> ksni::tray::Properties {
      ksni::tray::Properties {
        category: Category::Hardware,
        id: "tabletsupport".to_owned(),
        item_is_menu: true,
        icon_pixmap: vec![load_icon()],
        title: "Screen Orientation".to_owned(),
        tool_tip: ToolTip {
          icon_name: "tabletsupport".to_string(),
          icon_pixmap: vec![load_icon()],
          title: "Screen Orientation".to_string(),
          description: "Click to toggle orientation lock.".to_string(),
        },
        ..Default::default()
      }
    }

    fn menu_properties() -> ksni::menu::Properties {
      ksni::menu::Properties {
        ..Default::default()
      }
    }

    fn menu() -> Vec<ksni::menu::MenuItem> {
      vec![]
    }
  }
  ksni::run(Tray { state });
}

fn load_icon() -> Icon {
  let data = include_bytes!("../res/rotation-normal.png");
  let dec = png::Decoder::new(&data[..]);
  let (i, mut r) = dec.read_info().unwrap();
  let mut buf = vec![0; i.buffer_size()];
  r.next_frame(&mut buf).unwrap();
  for i in 0..buf.len() / 4 {
    let (r, a) = (buf[i * 4], buf[i * 4 + 3]);
    buf[i * 4] = a;
    buf[i * 4 + 3] = r;
  }
  Icon {
    width: i.width as i32,
    height: i.height as i32,
    data: buf,
  }
}