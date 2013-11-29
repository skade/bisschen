use bisschen::tags::Tags;
use super::lines::{Line,Lines,FlexString,Display};
use tmux::set;

impl Lines for Tags {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line] {
    self.iter()
        .skip(offset)
        .take(limit)
        .map(|x| {
          Line { fields: ~[x.str.to_owned()] }
        }).to_owned_vec()
  }

  fn handle_move(&mut self, line: uint) {
    let tag = self.idx(line);

    match tag {
      Some(t) => {
        set(~"BISSCHEN_CURRENT_TAG", t.str.clone());
      },
      None => {}
    }
  }

  fn display(&self) -> ~[Display] {
    ~[FlexString]
  }
}