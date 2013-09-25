use c::termbox::*;

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
        let event = tb_event { event_type: 0,
                               modifier: 0,
                               key: 0,
                               ch: 0,
                               w: 0,
                               h: 0 };
        tb_poll_event(&event);

        self.handle_key(event.key.to_int());

        if event.key == 0x0D {
          return;
        }
      }
    }
  }

  fn handle_key(&self, key: int) {
    self.channel.send(key);
  }
}
