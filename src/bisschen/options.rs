use extra::getopts::*;

use std::os;

use bisschen::database::*;

pub fn parse_opts() -> CommandLine {
  let args = os::args();

  let program = args[0].clone();

  let opts = ~[
    optopt("d"),
    optopt("database"),
    optopt("q"),
    optopt("query"),
    optflag("h"),
    optflag("help"),
  ];

  let matches = match getopts(args.tail(), opts) {
    Ok(m) => { m }
    Err(f) => { fail!(f.to_err_msg()) }
  };

  CommandLine { name: program, matches: matches }
}

struct CommandLine {
  name: ~str,
  matches: Matches
}

impl CommandLine {
  pub fn database(&self) -> Database {
    let database_folder = self.matches.opts_str([~"d", ~"database"]);
    Database::open(database_folder)
  }

  pub fn query_string(&self) -> Option<~str> {
    self.matches.opts_str([~"q", ~"query"])
  }
}