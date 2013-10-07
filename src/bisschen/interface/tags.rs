use bisschen::tags::*;
use super::lines::*;
use tmux::*;

impl Lines for Tags {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line] {
    self.iter()
        .skip(offset)
        .take(limit)
        .map(|x| {
          Line { line: x.str.to_owned() }
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
}