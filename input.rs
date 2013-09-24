#[crate_type = "lib"];
#[link(name = "input", vers = "0.01")];

extern mod c;

use c::*;

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
        let key = ncurses::getch().to_int();
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