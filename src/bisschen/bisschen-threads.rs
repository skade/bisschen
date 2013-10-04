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
  let threads = database.query(query_string).threads();

  let list = List::new(threads);
  let mut interface: Interface<List<Threads>> = Interface::new(list);
  interface.init();

  loop {
    let event = Input.poll();
    match event {
      Left(kp) => {
        if kp.key == 0x0D {
          return;
        }
      },
      Right(_) => { },
    }

    interface.handle_event(event);
  }
}
