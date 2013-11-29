extern mod bisschen;
extern mod termbox;
extern mod extra;

use termbox::Termbox;
use input::Input;
use interface::{Interface,List};
use options::parse_opts;
use bisschen::threads::Threads;

pub mod input;
pub mod interface;
pub mod options;
pub mod tmux;

fn main() {
  let opts = parse_opts();

  let mut termbox = Termbox::new();
  termbox.start_boxing();

  let database = opts.database();
  let query_string = opts.query_string();
  let threads = database.query(query_string).threads();

  let list = List::new(threads);
  let mut interface: Interface<List<Threads>> = Interface::new(list);
  interface.init();

  loop {
    let event = Input.poll();
    interface.handle_event(event);
  }
}
