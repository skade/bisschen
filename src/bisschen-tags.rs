extern mod bisschen;

use bisschen::database::*;
use bisschen::input::*;
use bisschen::termbox::*;
use bisschen::interface::*;
use std::comm::*;

fn main() {
  let mut termbox = Termbox::new();
  termbox.start_boxing();

  let (port, chan) = stream::<int>();
  let input = Input::new(chan);

  let database = Database::open("/Users/skade/Mail");
  let tags = database.tags();
  let list = List::new(tags);
  let mut interface: Interface<List<Tags>> = Interface::new(list, port);
  do spawn {
    input.run();
  }
  interface.run();
}
