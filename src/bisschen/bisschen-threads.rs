extern mod bisschen;
extern mod termbox;
extern mod extra;

use bisschen::database::*;
use bisschen::threads::*;
use termbox::*;
use input::*;
use interface::*;

use extra::getopts::*;

use std::comm::*;
use std::os;

pub mod input;
pub mod interface;

fn parse_opts() -> Matches {
  let args = os::args();

  let program = args[0].clone();

  let opts = ~[
    optopt("o"),
    optopt("d"),
    optopt("database"),
    optopt("q"),
    optopt("query"),
    optflag("h"),
    optflag("help"),
  ];

  match getopts(args.tail(), opts) {
    Ok(m) => { m }
    Err(f) => { fail!(f.to_err_msg()) }
  }
}
fn main() {
  let opts = parse_opts();

  let database_folder = opts.opts_str([~"d", ~"database"]);
  let database = Database::open(database_folder);

  let mut termbox = Termbox::new();
  termbox.start_boxing();

  let (port, chan) = stream::<Either<KeyPress, Resize>>();
  let input = Input::new(chan);

  let query_string = opts.opts_str([~"q", ~"query"]);
  let threads = database.query(query_string).threads();
  let list = List::new(threads);
  let mut interface: Interface<List<Threads>> = Interface::new(list, port);

  do spawn {
    input.run();
  }
  interface.run();
}
