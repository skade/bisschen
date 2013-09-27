#[crate_type = "lib"];
#[link(name = "termbox",
       vers = "0.1-pre",
       url = "")];

use cbits::termbox::*;

pub mod cbits;


pub struct Termbox {
  on: bool,
}

pub struct KeyPress {
  modifier: u8,
  key: u16,
  ch: u32
}

impl KeyPress {
  fn new(event: tb_event) -> KeyPress {
    KeyPress { modifier: event.modifier, key: event.key, ch: event.ch }
  }
}

pub struct Resize {
  w: i32,
  h: i32
}

impl Resize {
  fn new(event: tb_event) -> Resize {
    Resize { w: event.w, h: event.h }
  }
}

#[fixed_stack_segment]
pub fn poll_event() -> Either<KeyPress, Resize> {
  let event = tb_event { event_type: 0,
                         modifier: 0,
                         key: 0,
                         ch: 0,
                         w: 0,
                         h: 0 };

  unsafe {
    let event_type = tb_poll_event(&event);
    match event_type {
      TB_EVENT_KEY => { Left(KeyPress::new(event)) },
      TB_EVENT_RESIZE => { Right(Resize::new(event)) }
    }
  }
}

impl Termbox {
  pub fn new() -> Termbox {
    Termbox { on: false }
  }

  #[fixed_stack_segment]
  pub fn start_boxing(&mut self) {
    unsafe {
      match tb_init() {
        0 => { self.on = true }
        _ => { fail!("something went wrong when initalizing termbox!") }
      }
    }
  }

  #[fixed_stack_segment]
  pub fn stop_boxing(&mut self) {
    if self.on {
      unsafe { tb_shutdown(); }
      self.on = false;
    }
  }
}

impl Drop for Termbox {
  #[fixed_stack_segment]
  fn drop(&mut self) {
    self.stop_boxing();
  }
}
