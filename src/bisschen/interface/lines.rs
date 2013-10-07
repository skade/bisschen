pub struct Line {
  line: ~str,
}

pub trait Lines {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line];
  fn handle_move(&mut self, line: uint);
}
