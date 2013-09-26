use termbox::*;

struct Input {
  channel: Chan<int>,
}

impl Input {
  pub fn new(channel: Chan<int>) -> Input {
    Input { channel: channel }
  }

  pub fn run(&self) {
    loop {
      let event = poll_event();
      match event {
        Left(kp) => {
          self.handle_key(kp);

          if kp.key == 0x0D {
            return;
          }
        },
        Right(resize) => { },
      }

    }
  }

  fn handle_key(&self, event: KeyPress) {
    self.channel.send(event.key.to_int());
  }
}
