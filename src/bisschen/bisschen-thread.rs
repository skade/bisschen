extern mod bisschen;
extern mod termbox;
extern mod extra;

use termbox::*;
use input::*;
use interface::*;
use options::*;
use bisschen::threads::*;

use std::os::getenv;
use std::run::*;

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
    match event {
      Left(kp) => {
        if kp.key == 0x0D {
          return;
        }
        if kp.key == 0x7F {
          let env = getenv("BISSCHEN_LAST_PROGRAM");
          match env {
            Some(program) => {
              Process::new("tmux", [~"respawn-pane", ~"-k", program.clone()], ProcessOptions::new());
              debug2!("Program: {:?}", program);
            },
            None => {
              debug2!("No last program found");
            }
          }
        }
      },
      Right(_) => { },
    }

    interface.handle_event(event);
  }
}
