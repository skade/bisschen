use bisschen::threads::*;
use super::lines::*;
use tmux::*;

impl Lines for Threads {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line] {
    self.iter()
        .skip(offset)
        .take(limit)
        .map(|x| Line { fields: ~[x.subject()] } )
        .to_owned_vec()
  }

  fn handle_move(&mut self, line: uint) {
    let thread = self.idx(line);

    match thread {
      Some(t) => {
        set(~"BISSCHEN_CURRENT_THREAD", t.id());
      },
      None => {}
    }
  }

  fn display(&self) -> ~[Display] {
    ~[FlexString]
  }
}
