#[crate_type = "lib"];
#[link(name = "curses", vers = "0.01")];

extern mod c;

use c::*;

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
      ncurses::initscr();
      ncurses::noecho();
    }
  }

  #[fixed_stack_segment]
  #[inline(never)]
  pub fn stop_cursing(&mut self) {
    if self.cursing {
      self.cursing = false;
      unsafe { ncurses::endwin() };
    }
  }
}

impl Drop for Curses {
  fn drop(&mut self) {
    self.stop_cursing();
    println("stopped cursing");
  }
}