extern mod termbox;
extern mod bisschen;

use termbox::*;
use termbox::cbits::termbox::*;
use termbox::cbits::termbox::{tb_cell};

use self::lines::*;
use std::iter::range_inclusive;

pub mod lines;
pub mod tags;
pub mod threads;
pub mod thread;

struct Cursor {
  col: i32,
  line: i32
}

pub struct List<T> {
  contents: T,
  // This is the curses cursor!
  cursor: Cursor,
  selection: uint,
  offset: i32
}

trait Drawable {
  fn draw(&mut self);
  fn redraw(&mut self);
}

trait EventHandler {
  fn handle_event(&mut self, event: Either<KeyPress, Resize>) {
    match event {
      Left(k) => { self.handle_keypress(k) },
      Right(r) => { self.handle_resize(r) },
    }
  }

  fn handle_keypress(&mut self, key_press: KeyPress);
  fn handle_resize(&mut self, key_press: Resize);
}

impl Cursor {
  fn next_line(&mut self) {
    self.line += 1;
  }

  fn reset(&mut self) {
    self.line = 0;
    self.col = 0;
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

impl<T: Lines> EventHandler for List<T> {
  fn handle_keypress(&mut self, key_press: KeyPress) {
    match key_press.ch {
      'j' => {
        self.move_down();
        self.contents.handle_move(self.selection);
      },
      'k' => {
        self.move_up();
        self.contents.handle_move(self.selection);
      },
      _ => {}
    }
  }

  fn handle_resize(&mut self, _: Resize) { }
}

#[fixed_stack_segment]
fn height() -> i32 {
  unsafe { tb_height() }
}

#[fixed_stack_segment]
fn width() -> i32 {
  unsafe { tb_width() }
}

impl<T: Lines> List<T> {
  pub fn new(contents: T) -> List<T> {
    List { contents: contents,
           cursor: Cursor { col: 0, line: 0 },
           selection: 0,
           offset: 0 }
  }

  fn display_lines(&mut self) {
    let lines = self.contents.lines(self.offset as uint, height() as uint);
    let cols = self.contents.display();

    let mut offset = 0;
    for (i, &display_type) in cols.iter().enumerate() {
      let col = lines.iter().map(|x| x.fields[i].clone()).to_owned_vec();
      offset += self.print_cell(display_type, col, offset);
    }
  }

  fn print_cell(&self, display_type: Display, lines: ~[~str], offset: uint) -> uint {
    match display_type {
      Tree => { self.print_tree(lines, offset) },
      FlexString => { self.print_flexstring(lines,offset); 0 }
      _ => { 0 }
    }
  }

  fn print_tree(&self, lines: ~[~str], offset: uint) -> uint {
    let levels: ~[uint] = lines.iter().map(|e| from_str(e.clone()).unwrap()).to_owned_vec();
    let max_level = levels.iter().max().unwrap();

    for cur_level in range_inclusive(0u, *max_level) {
      for (row, level) in levels.iter().enumerate() {
        if *level >= cur_level + 1 {
          if *level == cur_level + 1 {
            self.put_str(cur_level.to_i32().unwrap(), row.to_i32().unwrap(), ~"└");
          } else {
            self.put_str(cur_level.to_i32().unwrap(), row.to_i32().unwrap(), ~"|");
          }
        } else {
          self.put_str(cur_level.to_i32().unwrap(), row.to_i32().unwrap(), ~" ");
        }
      }
    }
    *max_level
  }

  fn print_flexstring(&self, lines: ~[~str], offset: uint) -> uint {
    let mut width = offset;

    for (row, line) in lines.iter().enumerate() {
      if line.len() > width {
        width = line.len();
      }
      self.put_str(offset.to_i32().unwrap(), row.to_i32().unwrap(), line.to_owned());
    }
    width
  }

  fn put_str(&self, col: i32, row: i32, string: ~str) {
    for (offset, ch) in string.iter().enumerate() {
      let cell;
      if self.selection == row as uint {
        cell = tb_cell { character: ch as u32,
                                        foreground: 5,
                                        background: 3 };
      } else {
        cell = tb_cell { character: ch as u32,
                            foreground: 4,
                            background: 8 };
      }
      self.put_cell(col + offset.to_i32().unwrap(), row.to_i32().unwrap(), cell);
    }
  }

  #[fixed_stack_segment]
  fn put_cell(&self, col: i32, row: i32, cell: tb_cell) {
    unsafe { tb_put_cell(col, row, &cell); }
  }

  #[fixed_stack_segment]
  fn clear(&mut self) {
    unsafe { tb_clear(); }
    self.cursor.reset()
  }

  #[fixed_stack_segment]
  fn refresh(&self) {
    unsafe { tb_present(); }
  }

  fn move_down(&mut self) {
    if self.selection < (height() - 1) as uint {
      self.selection += 1;
    } else {
      self.offset += 1;
    }
  }

  fn move_up(&mut self) {
    if self.selection > 0 {
      self.selection -= 1;
    } else {
      if !(self.offset == 0) {
        self.offset -= 1;
      }
    }
  }
}

pub struct Interface<T> {
  view: T,
  active: bool,
  redraw_count: int
}

impl<T: Drawable + EventHandler> Interface<T> {
  pub fn new(view: T) -> Interface<T> {

    Interface { view: view,
                active: false,
                redraw_count: 0 }
  }

  pub fn init(&mut self) {
    self.view.draw();
  }

  pub fn handle_event(&mut self, event: Either<KeyPress, Resize>) {
    self.view.draw();
    self.view.handle_event(event);
    self.view.redraw();
  }
}
