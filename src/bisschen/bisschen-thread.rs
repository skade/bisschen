extern mod bisschen;
extern mod termbox;
extern mod extra;

use termbox::*;
use input::*;
use interface::*;
use options::*;
use bisschen::threads::*;

pub mod input;
pub mod interface;
pub mod options;

fn main() {
  let opts = parse_opts();

  let mut termbox = Termbox::new();
  termbox.start_boxing();

  let database = opts.database();
  let query_string = opts.query_string();

  let mut threads = database.query(query_string).threads();
  let thread = threads.get_next_thread();

  let list = List::new(thread.unwrap());
  let mut interface: Interface<List<Thread>> = Interface::new(list);
  interface.init();

  loop {
    let event = Input.poll();
    interface.handle_event(event);
  }
}
