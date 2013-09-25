use c::ncurses::*;

struct Curses {
  cursing: bool,
}

impl Curses {
  pub fn new() -> Curses {
    Curses { cursing: false }
  }

  #[fixed_stack_segment]
  pub fn start_cursing(&mut self) {
    self.cursing = true;
    unsafe {
      initscr();
      start_color();
      noecho();
    }
  }

  #[fixed_stack_segment]
  #[inline(never)]
  pub fn stop_cursing(&mut self) {
    if self.cursing {
      self.cursing = false;
      unsafe { endwin() };
    }
  }
}

impl Drop for Curses {
  fn drop(&mut self) {
    self.stop_cursing();
    println("stopped cursing");
  }
}