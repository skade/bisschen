use termbox::{poll_event,Action};

pub struct Input;

impl Input {
//  pub fn run(&self) {
//    loop {
//      let event = self.poll();
//      //self.send_event(event);
//    }
//  }

  pub fn poll(&self) -> Action {
    poll_event()
  }
}
