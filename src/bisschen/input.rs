use termbox::*;

pub struct Input {
  channel: Chan<Either<KeyPress, Resize>>,
}

impl Input {
  pub fn new(channel: Chan<Either<KeyPress, Resize>>) -> Input {
    Input { channel: channel }
  }

  pub fn run(&self) {
    loop {
      let event = poll_event();
      self.send_event(event);
      match event {
        Left(kp) => {
          if kp.key == 0x0D {
            return;
          }
        },
        Right(_) => { },
      }
    }
  }

  fn send_event(&self, event: Either<KeyPress, Resize>) {
    self.channel.send(event);
  }
}
