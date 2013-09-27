extern mod bisschen;
extern mod termbox;

use bisschen::database::*;
use bisschen::tags::*;
use termbox::*;
use input::*;
use interface::*;

use std::comm::*;

pub mod input;
pub mod interface;

fn main() {
  let mut termbox = Termbox::new();
  termbox.start_boxing();

  let (port, chan) = stream::<Either<KeyPress, Resize>>();
  let input = Input::new(chan);

  let database = Database::open(None);
  let tags = database.tags();
  let list = List::new(tags);
  let mut interface: Interface<List<Tags>> = Interface::new(list, port);
  do spawn {
    input.run();
  }
  interface.run();
}
