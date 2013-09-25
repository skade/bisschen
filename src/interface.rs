use c::ncurses::*;
use database::*;
use std::comm::*;
use std::c_str::*;

struct Cursor {
  col: i32,
  line: i32
}

struct List<T> {
  contents: T,
  // This is the curses cursor!
  cursor: Cursor,
  selection: int
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

trait KeyHandler {
  fn handle_key(&self, key: int);
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

impl Cursor {
  #[fixed_stack_segment]
  fn move_to(&self) {
    unsafe {
      move(self.line, self.col);
    }
  }

  fn next_line(&mut self) {
    self.line += 1;
    self.move_to()
  }

  fn reset(&mut self) {
    self.line = 0;
    self.col = 0;
    self.move_to()
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

impl<T> KeyHandler for List<T> {
  fn handle_key(&self, key: int) {
    printstr(key.to_str())
  }
}

#[fixed_stack_segment]
fn printstr(str: &str) {
  do str.with_c_str
    |c_string| { unsafe { printw(c_string) } };
}

impl<T: Lines> List<T> {
  pub fn new(contents: T) -> List<T> {
    List { contents: contents,
           cursor: Cursor { col: 0, line: 0 },
           selection: 0 }
  }

  fn display_lines(&mut self) {
    let lines = self.contents.lines();
    for line in lines.iter() {
      printstr(line.line);
      self.cursor.next_line()
    }
  }

  #[fixed_stack_segment]
  fn print_line(&self, str: &str) {
    do str.with_c_str
      |c_string| { unsafe { printw(c_string) } };
  }

  #[fixed_stack_segment]
  fn clear(&mut self) {
    unsafe { clear(); }
    self.cursor.reset()
  }

  #[fixed_stack_segment]
  fn refresh(&self) {
    unsafe { refresh(); }
  }
}

struct Interface<T> {
  port: Port<int>,
  view: T,
  active: bool,
  redraw_count: int
}

impl<T: Drawable + KeyHandler> Interface<T> {
  pub fn new(view: T, port: Port<int>) -> Interface<T> {

    Interface { port: port,
                view: view,
                active: false,
                redraw_count: 0 }
  }

  #[fixed_stack_segment]
  pub fn run(&mut self) {
    self.view.draw();
    loop {
      let val = self.port.recv();

      self.view.handle_key(val);

      if val == 10 {
        return;
      }
      self.view.redraw();
    }
  }
}