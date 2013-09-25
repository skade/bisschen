use c::termbox::*;

struct Termbox {
  on: bool,
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
