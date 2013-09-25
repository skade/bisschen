use c::ncurses::*;

struct Input {
  channel: Chan<int>,
}

impl Input {
  pub fn new(channel: Chan<int>) -> Input {
    Input { channel: channel }
  }

  #[fixed_stack_segment]
  pub fn run(&self) {
    loop {
      unsafe {
        let key = getch().to_int();
        self.handle_key(key);
        if key == 10 {
          return;
        }
      }
    }
  }

  fn handle_key(&self, key: int) {
    self.channel.send(key);
  }
}