extern mod c;
extern mod database;
extern mod input;
extern mod curses;

use c::*;
use database::*;
use std::comm::*;
use std::c_str::*;
use std::util::*;
use input::*;
use curses::*;

struct Position {
  col: i32,
  line: i32
}

struct List<T> {
  contents: T,
  position: Position,
}

struct Line {
  line: ~str,
}

trait Lines {
  fn lines(&self) -> ~[Line];
}

trait Drawable {
  fn draw(&mut self);
  fn redraw(&mut self);
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

impl Lines for Threads {
  fn lines(&self) -> ~[Line] {
    self.map(|x| x.subject())
        .map(|c_string| {
          match c_string.as_str() {
            Some(str) => { Line { line: str.to_owned() } }
            None => { fail!("Threads should never yield illegal subjects!") }
          }
        }).to_owned_vec()
  }
}


impl Position {
  #[fixed_stack_segment]
  fn move_to(&self) {
    unsafe {
      ncurses::move(self.line, self.col);
    }
  }
}

impl<T: Lines> Drawable for List<T> {
  fn draw(&mut self) {
    self.display_lines();
    self.refresh();
  }

  fn redraw(&mut self) {
    self.clear();
    self.draw();
  }
}


#[fixed_stack_segment]
fn printstr(str: &str) {
  do str.with_c_str
    |c_string| { unsafe { ncurses::printw(c_string) } };
}

impl<T: Lines> List<T> {
  fn new(contents: T) -> List<T> {
    List { contents: contents, position: Position { col: 0, line: 0 } }
  }

  fn display_lines(&mut self) {
    let lines = self.contents.lines();
    for line in lines.iter() {
      printstr(line.line);
      self.position.line += 1;
      self.position.move_to()
    }
  }

  #[fixed_stack_segment]
  fn print_line(&self, str: &str) {
    do str.with_c_str
      |c_string| { unsafe { ncurses::printw(c_string) } };
  }

  #[fixed_stack_segment]
  fn clear(&mut self) {
    unsafe { ncurses::clear(); }
    self.position = Position { col: 0, line: 0 };
    self.position.move_to()
  }

  #[fixed_stack_segment]
  fn refresh(&self) {
    unsafe { ncurses::refresh(); }
  }
}

struct Interface<T> {
  port: Port<int>,
  view: T,
  active: bool,
  redraw_count: int
}

impl<T: Drawable> Interface<T> {
  fn new(view: T, port: Port<int>) -> Interface<T> {

    Interface { port: port,
                view: view,
                active: false,
                redraw_count: 0 }
  }

  #[fixed_stack_segment]
  fn run(&mut self) {
    self.view.draw();
    loop {
      let val = self.port.recv();

      if val == 10 {
        return;
      }
      self.view.redraw();
    }
  }
}

fn main() {
  let mut curses = Curses::new();
  curses.start_cursing();

  let (port, chan) = stream::<int>();
  let input = Input::new(chan);

  let database = Database::open("/Users/skade/Mail");
  let tags = id(database.tags());
  let list = List::new(tags);
  let mut interface: Interface<List<Tags>> = Interface::new(list, port);
  do spawn {
    input.run();
  }
  interface.run();
}