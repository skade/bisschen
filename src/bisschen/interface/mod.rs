extern mod termbox;
extern mod bisschen;

use termbox::*;
use termbox::cbits::termbox::*;
use termbox::cbits::termbox::{tb_cell};

use self::lines::*;

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
    for line in lines.iter() {
      self.print_line(line, self.cursor.line as uint);
      self.cursor.next_line()
    }
  }

  #[fixed_stack_segment]
  fn print_line(&mut self, line: &Line, no: uint) {
    let rest = width() as uint - line.fields[0].len();
    let mut bytes = line.fields[0].as_bytes().to_owned();
    bytes.grow_fn(rest, |_| ' ' as u8);

    for (offset, ch) in bytes.iter().enumerate() {
      let cell;
      if self.selection == no {
        cell = tb_cell { character: *ch as u32,
                             foreground: 5,
                             background: 3 };
      } else {
         cell = tb_cell { character: *ch as u32,
                             foreground: 4,
                             background: 8 };
      }

      unsafe { tb_put_cell(offset.to_i32(), no.to_i32(), &cell); }
    }
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