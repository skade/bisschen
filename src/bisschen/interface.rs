extern mod termbox;
extern mod bisschen;

use termbox::*;
use termbox::cbits::termbox::*;
use termbox::cbits::termbox::{tb_cell};

use bisschen::tags::*;
use bisschen::threads::*;

use std::run::*;

use std::path::Path;
use std::rt::io::file::{FileInfo};
use std::rt::io::Writer;
use std::rt::io::{Create};

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

struct Line {
  line: ~str,
}

trait Lines {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line];
  fn handle_selection(&mut self, line: uint);
  fn handle_reply(&mut self, line: uint);
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

impl Lines for Tags {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line] {
    self.iter()
        .skip(offset)
        .take(limit)
        .map(|x| {
          Line { line: x.str.to_owned() }
        }).to_owned_vec()
  }

  fn handle_selection(&mut self, line: uint) {
    let tag = self.idx(line);

    match tag {
      Some(t) => {
        let mut last = Process::new("tmux", [~"set-environment", ~"BISSCHEN_LAST_PROGRAM", ~"build/bisschen-tags"], ProcessOptions::new());
        last.finish();
        let mut tag = Process::new("tmux", [~"set-environment", ~"BISSCHEN_CURRENT_TAG", t.str.clone()], ProcessOptions::new());
        tag.finish();
        let query = ~"build/bisschen-threads --query tag:" + t.str;
        Process::new("tmux", [~"respawn-pane", ~"-k", query], ProcessOptions::new());
      },
      None => {},
    }
  }

  fn handle_reply(&mut self, _line: uint) {
  }
}

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

  fn handle_selection(&mut self, line: uint) {
    let thread = self.idx(line);

    match thread {
      Some(t) => {
        let mut last = Process::new("tmux", [~"set-environment", ~"BISSCHEN_LAST_PROGRAM", ~"build/bisschen-threads"], ProcessOptions::new());
        last.finish();
        let mut tag = Process::new("tmux", [~"set-environment", ~"BISSCHEN_CURRENT_THREAD", t.id()], ProcessOptions::new());
        tag.finish();
        let query = ~"build/bisschen-thread --query thread:" + t.id();
        Process::new("tmux", [~"respawn-pane", ~"-k", query], ProcessOptions::new());
      },
      None => {},
    }
  }
  fn handle_reply(&mut self, _line: uint) {
  }
}

impl Lines for Thread {
  fn lines(&mut self, offset: uint, limit: uint) -> ~[Line] {
    let mut messages = self.messages();
    messages
        .iter()
        .skip(offset)
        .take(limit)
        .map(|x| x.subject())
        .map(|c_string| {
          match c_string.as_str() {
            Some(str) => { Line { line: str.to_owned() } }
            None => { fail!("Messages should never yield illegal subjects!") }
          }
        }).to_owned_vec()
  }

  fn handle_selection(&mut self, line: uint) {
    let mut messages = self.messages();
    let single_message = messages.iter().skip(line).take(1).to_owned_vec();
    let m = single_message[0];

    debug2!("message id: {:?}", m.id());
    let mut last = Process::new("tmux", [~"set-environment", ~"BISSCHEN_LAST_PROGRAM", ~"build/bisschen-thread"], ProcessOptions::new());
    last.finish();
    let mut tag = Process::new("tmux", [~"set-environment", ~"BISSCHEN_CURRENT_MESSAGE", m.id()], ProcessOptions::new());
    tag.finish();
    let mut pane_select = Process::new("tmux", [~"select-pane", ~"-t", ~":.1"], ProcessOptions::new());
    let pane_present = pane_select.finish();

    debug2!("pane_found? {:?}", pane_present);

    let program = "vim " + m.filename() + " -c \":silent! %s/<\\_.\\{-1,\\}>//g\" \"+set nowarn\" \"+set filetype=mail\" \"+set foldmethod=syntax\" \"+set noma\" \"+set buftype=nofile\" \"+setlocal noswapfile\"";

    if pane_present == 1 {
      Process::new("tmux", [~"split-window", ~"-v", program], ProcessOptions::new());
      Process::new("tmux", [~"select-pane", ~"-t", ~":.0"], ProcessOptions::new());
    } else {
      Process::new("tmux", [~"respawn-pane", ~"-k", ~"-t 1", program], ProcessOptions::new());
      Process::new("tmux", [~"select-pane", ~"-t", ~":.0"], ProcessOptions::new());
    }
  }

  fn handle_reply(&mut self, line: uint) {
    let mut messages = self.messages();
    let single_message = messages.iter().skip(line).take(1).to_owned_vec();
    let m = single_message[0];

    debug2!("message id: {:?}", m.id());
    let mut last = Process::new("tmux", [~"set-environment", ~"BISSCHEN_LAST_PROGRAM", ~"build/bisschen-thread"], ProcessOptions::new());
    last.finish();
    let mut tag = Process::new("tmux", [~"set-environment", ~"BISSCHEN_CURRENT_MESSAGE", m.id()], ProcessOptions::new());
    tag.finish();
    let mut pane_select = Process::new("tmux", [~"select-pane", ~"-t", ~":.1"], ProcessOptions::new());
    let pane_present = pane_select.finish();

    debug2!("pane_found? {:?}", pane_present);

    let draft_file_path = &Path("draft.mail");
    let mut draft_mail = Process::new("notmuch", [~"reply", "id:" + m.id()], ProcessOptions::new());
    let output = draft_mail.finish_with_output();
    let mut writer = draft_file_path.open_writer(Create).unwrap();
    (&mut writer as &mut Writer).write(output.output);

    let program = ~"vim draft.mail -c \":silent! %s/<\\_.\\{-1,\\}>//g\" \"+set nowarn\" \"+set filetype=mail\" \"+set foldmethod=syntax\"  \"+set buftype=nofile\" \"+setlocal noswapfile\"";

    if pane_present == 1 {
      Process::new("tmux", [~"split-window", ~"-v", program], ProcessOptions::new());
      Process::new("tmux", [~"select-pane", ~"-t", ~":.0"], ProcessOptions::new());
    } else {
      Process::new("tmux", [~"respawn-pane", ~"-k", ~"-t 1", program], ProcessOptions::new());
      Process::new("tmux", [~"select-pane", ~"-t", ~":.0"], ProcessOptions::new());
    }
  }
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
      'j' => { self.move_down(); },
      'k' => { self.move_up(); },
      'r' => { self.contents.handle_reply(self.selection); },
      _ => {}
    }

    match key_press.key {
      0x20 => { self.contents.handle_selection(self.selection); },
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
    let rest = width() as uint - line.line.len();
    let mut bytes = line.line.as_bytes().to_owned();
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
