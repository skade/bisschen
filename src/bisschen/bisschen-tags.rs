extern crate bisschen;
extern crate termbox;
extern crate extra;
extern crate getopts;

use termbox::Termbox;
use input::Input;
use interface::{Interface,List};
use options::parse_opts;
use bisschen::tags::Tags;

pub mod input;
pub mod interface;
pub mod options;
pub mod tmux;

fn main() {
  let opts = parse_opts();

  let mut termbox = Termbox::new();
  termbox.start_boxing();

  let database = opts.database();
  let tags = database.tags();
  let list = List::new(tags);
  let mut interface: Interface<List<Tags>> = Interface::new(list);
  interface.init();

  loop {
    let event = Input.poll();
    interface.handle_event(event);
  }
}
