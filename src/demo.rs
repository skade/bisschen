extern mod bisschen;

use bisschen::database::*;
use bisschen::termbox::*;
use bisschen::c::termbox::*;

struct List<T> {
  contents: T,
}

struct Line {
  line: ~str,
}

trait Lines {
  fn lines(&self) -> ~[Line];
}

impl Lines for Tags {
  fn lines(&self) -> ~[Line] {
    self.map(|c_string| {
      match c_string.as_str() {
        Some(str) => { Line { line: str.to_owned() } }
        None => { fail!("Tags should never yield illegal strings!") }
      }
    }).to_owned_vec()
  }
}

#[fixed_stack_segment]
fn print_line(line: &Line, no: uint) {
  for (offset, ch) in line.line.char_offset_iter() {
    let cell = tb_cell { character: ch as u32,
                         foreground: 5,
                         background: 3 };

    unsafe { tb_put_cell(offset.to_i32(), no.to_i32(), &cell); }
  }
  unsafe { tb_present() };
}

impl<T: Lines> List<T> {
  fn new(contents: T) -> List<T> {
    List { contents: contents }
  }

  #[fixed_stack_segment]
  fn print_lines(&mut self) {
    unsafe { tb_clear() };
    let lines = self.contents.lines();
    for (index, line) in lines.iter().enumerate() {
      print_line(line, index);
    }
    let mut event = tb_event { event_type: 0,
                               modifier: 0,
                               key: 0,
                               ch: 0,
                               w: 0,
                               h: 0};

    unsafe { tb_poll_event(&event) };
  }
}

fn main() {
  let mut termbox = Termbox::new();
  termbox.start_boxing();
  let database = Database::open("/Users/skade/Mail");
  let tags = database.tags();
  let mut list: List<Tags> = List::new(tags);
  list.print_lines()
}
