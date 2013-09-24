extern mod c;
extern mod database;

use c::*;
use database::*;
use std::comm::*;
use std::c_str::*;
use std::util::*;

struct Position {
  col: i32,
  line: i32
}

struct List<T> {
  contents: T,
  position: Position,
}

trait Lines {
  fn lines<'a>(&'a mut self) -> &'a mut Iterator<CString>;
}

trait Drawable {
  fn draw(&mut self);
  fn redraw(&mut self);
}

impl Lines for Tags {
  fn lines<'a>(&'a mut self) -> &'a mut Iterator<CString> {
    self as &mut Iterator<CString>
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
    for line in self.contents.lines() {
      match line.as_str() {
        Some(str) => { printstr(str) }
        None => { }
      }
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

struct Curse {
  cursing: bool,
}

impl Curse {
  fn new() -> Curse {
    Curse { cursing: false }
  }

  #[fixed_stack_segment]
  fn start_cursing(&mut self) {
    self.cursing = true;
    unsafe {
      ncurses::initscr();
      ncurses::noecho();
    }
  }

  #[fixed_stack_segment]
  #[inline(never)]
  fn stop_cursing(&mut self) {
    if self.cursing {
      self.cursing = false;
      unsafe { ncurses::endwin() };
    }
  }
}

impl Drop for Curse {
  fn drop(&mut self) {
    self.stop_cursing();
    println("stopped cursing");
  }
}

struct Input {
  channel: Chan<int>,
}

impl Input {
  fn new(channel: Chan<int>) -> Input {
    Input { channel: channel }
  }

  #[fixed_stack_segment]
  fn run(&self) {
    loop {
      unsafe {
        let key = ncurses::getch().to_int();
        self.handle_key(key);
        if key == 10 {
          return;
        }
      }
    }
  }

  fn handle_key(&self, key: int) {
    self.channel.send(key);
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
  let mut curse = Curse::new();
  curse.start_cursing();

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