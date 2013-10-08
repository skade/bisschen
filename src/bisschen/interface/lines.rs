pub enum Display {
  FlexString,
  FixedString(int),
  Tree
}

pub struct Line {
  fields: ~[~str],
}

pub trait Lines {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line];
  fn display(&self) -> ~[Display];
  fn handle_move(&mut self, line: uint);
}
