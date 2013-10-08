use bisschen::threads::*;
use bisschen::messages::*;
use super::lines::*;
use tmux::*;

fn messages_to_lines(messages: &mut Messages, level: uint) -> ~[Line] {
  let mut res = ~[];
  for message in messages.iter() {
    res.push(Line { fields: ~[level.to_str(), message.subject()]});
    let msgs = messages_to_lines(&mut message.replies(), level + 1);
    res.push_all_move(msgs);
  }
  res
}

impl Lines for Thread {
  fn lines(&mut self, _offset: uint, _limit: uint) -> ~[Line] {
    let mut toplevel = self.toplevel_messages();

    messages_to_lines(&mut toplevel, 0)
  }

  fn handle_move(&mut self, line: uint) {
    let mut messages = self.messages();
    let single_message = messages.iter().skip(line).take(1).to_owned_vec();
    let m = single_message[0];

    set(~"BISSCHEN_CURRENT_MESSAGE", m.id());
    set(~"BISSCHEN_CURRENT_MESSAGE_FILE", m.filename());
  }

  fn display(&self) -> ~[Display] {
    ~[Tree, FlexString]
  }
}
