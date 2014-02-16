#[crate_type = "lib"];
#[crate_id = "termbox"];

use cbits::termbox::{tb_init,tb_shutdown,tb_poll_event,tb_event,TB_EVENT_KEY,TB_EVENT_RESIZE};
use std::char::from_u32;

pub mod cbits;

pub struct Termbox {
  on: bool,
}

pub struct KeyPress {
  modifier: u8,
  key: u16,
  ch: char
}

impl KeyPress {
  fn new(event: tb_event) -> KeyPress {
    let ch = from_u32(event.ch).expect("event sent invalid key");
    KeyPress { modifier: event.modifier, key: event.key, ch: ch }
  }
}

pub struct Resize {
  w: i32,
  h: i32
}

pub enum Action {
  KeyPress(KeyPress),
  Resize(Resize)
}

impl Resize {
  fn new(event: tb_event) -> Resize {
    Resize { w: event.w, h: event.h }
  }
}

pub fn poll_event() -> Action {
  let event = tb_event { event_type: 0,
                         modifier: 0,
                         key: 0,
                         ch: 0,
                         w: 0,
                         h: 0 };

  unsafe {
    let event_type = tb_poll_event(&event);
    match event_type {
      TB_EVENT_KEY => { KeyPress(KeyPress::new(event)) },
      TB_EVENT_RESIZE => { Resize(Resize::new(event)) }
    }
  }
}

impl Termbox {
  pub fn new() -> Termbox {
    Termbox { on: false }
  }

  pub fn start_boxing(&mut self) {
    unsafe {
      match tb_init() {
        0 => { self.on = true }
        _ => { fail!("something went wrong when initalizing termbox!") }
      }
    }
  }

  pub fn stop_boxing(&mut self) {
    if self.on {
      unsafe { tb_shutdown(); }
      self.on = false;
    }
  }
}

impl Drop for Termbox {
  fn drop(&mut self) {
    self.stop_boxing();
  }
}
