use c::ncurses::*;

struct Curses {
  cursing: bool,
  color_pairs: ColorPairs,
}

struct ColorPair {
  number: i16,
}

struct ColorPairs {
  pairs: ~[ColorPair],
}

impl ColorPair {
  #[fixed_stack_segment]
  fn new(pair: i16, foreground: i16, background: i16) -> ColorPair {
    unsafe { init_pair(pair, foreground, background) };
    ColorPair { number: pair }
  }
}

impl ColorPairs {
  fn new() -> ColorPairs {
    // as defined by curses
    let first_pair = ColorPair::new(0, COLOR_WHITE, COLOR_BLACK);
    ColorPairs { pairs: ~[first_pair] }
  }

  fn register(&mut self, foreground: i16, background: i16) -> ColorPair {
    let next_index = self.pairs.len() + 1;
    let pair = ColorPair::new(next_index.to_i16(), foreground, background);
    self.pairs.push(pair);
    pair
  }
}

impl Curses {
  pub fn new() -> Curses {
    Curses { cursing: false,
             color_pairs: ColorPairs::new() }
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
