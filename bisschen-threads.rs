extern mod c;
extern mod database;

use c::*;
use database::*;
use std::comm::*;

fn main() {
  let mut curse = Curse::new();
  curse.start_cursing();

  let (port, chan) = stream::<int>();
  let input = Input::new(chan);

  let database = Database::open("/Users/skade/Mail");
  let mut interface = Interface::new(database, port);
  do spawn {
    input.run();
  }
  interface.run();
}
