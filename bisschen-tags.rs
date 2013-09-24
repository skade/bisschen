extern mod c;
extern mod database;
extern mod input;
extern mod curses;
extern mod interface;

use c::*;
use database::*;
use std::comm::*;
use input::*;
use curses::*;
use interface::*;

fn main() {
  let mut curses = Curses::new();
  curses.start_cursing();

  let (port, chan) = stream::<int>();
  let input = Input::new(chan);

  let database = Database::open("/Users/skade/Mail");
  let tags = database.tags();
  let mut interface: Interface<List<Tags>> = Interface::new(list, port);
  do spawn {
    input.run();
  }
  interface.run();
}