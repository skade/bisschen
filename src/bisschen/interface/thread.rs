use bisschen::threads::*;
use super::lines::*;
use tmux::*;

impl Lines for Thread {
  fn lines(&mut self, _offset: uint, _limit: uint) -> ~[Line] {
    let mut messages = self.messages();
    //let path = ~[];
    messages
        .iter()
        .map(|x| Line { fields: ~[x.subject()] } )
        .to_owned_vec()
  }

  fn handle_move(&mut self, line: uint) {
    let mut messages = self.messages();
    let single_message = messages.iter().skip(line).take(1).to_owned_vec();
    let m = single_message[0];

    set(~"BISSCHEN_CURRENT_MESSAGE", m.id());
    set(~"BISSCHEN_CURRENT_MESSAGE_FILE", m.filename());
  }

}
