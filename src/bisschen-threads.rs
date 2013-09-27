extern mod bisschen;

use bisschen::database::*;
use bisschen::input::*;
use bisschen::termbox::*;
use bisschen::interface::*;
use bisschen::threads::*;

use std::comm::*;

fn main() {
  let mut termbox = Termbox::new();
  termbox.start_boxing();

  let (port, chan) = stream::<Either<KeyPress, Resize>>();
  let input = Input::new(chan);

  let database = Database::open(None);
  let threads = database.query("*").threads();
  let list = List::new(threads);
  let mut interface: Interface<List<Threads>> = Interface::new(list, port);

  do spawn {
    input.run();
  }
  interface.run();
}
