use bisschen::threads::*;
use std::run::*;
use super::lines::*;

impl Lines for Thread {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line] {
    let mut messages = self.messages();
    messages
        .iter()
        .skip(offset)
        .take(limit)
        .map(|x| x.subject())
        .map(|c_string| {
          match c_string.as_str() {
            Some(str) => { Line { line: str.to_owned() } }
            None => { fail!("Messages should never yield illegal subjects!") }
          }
        }).to_owned_vec()
  }

  fn handle_move(&mut self, line: uint) {
    let mut messages = self.messages();
    let single_message = messages.iter().skip(line).take(1).to_owned_vec();
    let m = single_message[0];

    let mut message = Process::new("tmux", [~"set", ~"@BISSCHEN_CURRENT_MESSAGE", m.id()], ProcessOptions::new());
    message.finish();
    let mut message = Process::new("tmux", [~"set", ~"@BISSCHEN_CURRENT_MESSAGE_FILE", m.filename()], ProcessOptions::new());
    message.finish();
  }

}
