extern mod c;
extern mod database;
extern mod input;
extern mod curses;
extern mod interface;

use database::*;
use std::comm::*;
use interface::*;

fn main() {
  let mut curses = Curses::new();
  curses.start_cursing();

  let (port, chan) = stream::<int>();
  let input = Input::new(chan);

  let database = Database::open("/Users/skade/Mail");
  let threads = database.query("*", Some(20), Some(0)).threads();
  let list = List::new(threads);
  let mut interface: Interface<List<Threads>> = Interface::new(list, port);

  do spawn {
    input.run();
  }
  interface.run();
}
