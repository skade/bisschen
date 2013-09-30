extern mod bisschen;
extern mod termbox;
extern mod extra;

use termbox::*;
use input::*;
use interface::*;
use options::*;
use bisschen::threads::*;

use std::comm::*;

pub mod input;
pub mod interface;
pub mod options;

fn main() {
  let opts = parse_opts();

  let mut termbox = Termbox::new();
  termbox.start_boxing();

  let database = opts.database();
  let query_string = opts.query_string();
  let threads = database.query(query_string).threads();

  let (port, chan) = stream::<Either<KeyPress, Resize>>();
  let input = Input::new(chan);

  let list = List::new(threads);
  let mut interface: Interface<List<Threads>> = Interface::new(list, port);

  do spawn {
    input.run();
  }
  interface.run();
}
