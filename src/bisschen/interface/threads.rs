use bisschen::threads::*;
use std::run::*;
use super::lines::*;

impl Lines for Threads {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line] {
    self.iter()
        .skip(offset)
        .take(limit)
        .map(|x| x.subject())
        .map(|c_string| {
          match c_string.as_str() {
            Some(str) => { Line { line: str.to_owned() } }
            None => { fail!("Threads should never yield illegal subjects!") }
          }
        }).to_owned_vec()
  }

  fn handle_move(&mut self, line: uint) {
    let thread = self.idx(line);

    match thread {
      Some(t) => {
        let mut tag = Process::new("tmux", [~"set", ~"@BISSCHEN_CURRENT_THREAD", t.id()], ProcessOptions::new());
        tag.finish();
      },
      None => {}
    }
  }

}
