extern mod c;
extern mod database;

use c::*;
use database::*;
use std::comm::*;

struct Position {
  col: i32,
  line: i32
}

struct List {
  position: Position,
}

impl Position {
  #[fixed_stack_segment]
  fn move_to(&self) {
    unsafe {
      ncurses::move(self.line, self.col);
    }
  }
}

impl List {
  fn new() -> List {
    List { position: Position { col: 0, line: 0} }
  }

  #[fixed_stack_segment]
  fn print_line(&mut self, str: &str) {
    do str.with_c_str
      |c_string| { unsafe { ncurses::printw(c_string) } };
    self.position.line += 1;
    self.position.move_to()
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

struct Interface {
  database: Database,
  port: Port<int>,
  view: List,
  active: bool,
  redraw_count: int
}

impl Interface {
  fn new(database: Database, port: Port<int>) -> Interface {
    Interface { database: database,
                port: port,
                view: List::new(),
                active: false,
                redraw_count: 0 }
  }

  fn redraw(&mut self) {
    self.view.clear();
    for tag in self.database.tags() {
      match tag.as_str() {
        Some(str) => { self.view.print_line(str) },
        None => { }
      }
    }
    self.view.print_line(self.redraw_count.to_str());
    self.redraw_count += 1;
    self.view.refresh();
  }

  #[fixed_stack_segment]
  fn run(&mut self) {
    loop {
      self.redraw();
      let val = self.port.recv();

      self.view.print_line(val.to_str());

      if val == 10 {
        return;
      }
    }
  }
}

fn main() {
  let mut curse = Curse::new();
  curse.start_cursing();

  let (port, chan) = stream::<int>();
  let input = Input::new(chan);

  let database = Database::open("/Users/skade/Mail");
  let mut interface = Interface::new(database, port);
  do spawn {
    input.run();
  }
  interface.run();
}