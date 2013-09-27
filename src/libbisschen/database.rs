use std::ptr;
use std::c_str::*;
use std::run::*;
use std::str::*;
use cbits::notmuch::*;
use tags::*;
use query::*;

pub struct Database {
  priv database: *notmuch_database_t,
}

fn get_database_path_from_cfg() -> ~str {
  let mut pr = Process::new("notmuch", [~"config", ~"get", ~"database.path"], ProcessOptions::new());
  let output = pr.finish_with_output();

  let utf8string = from_utf8(output.output);
  utf8string.trim().to_owned()
}

impl Database {
  pub fn new(database: *notmuch_database_t) -> Database {
    Database { database: database }
  }

  #[fixed_stack_segment]
  pub fn open(path: Option<~str>) -> Database {
    let database_path = match path {
      Some(str) => { str },
      None => { get_database_path_from_cfg() }
    };
    do database_path.with_c_str |c_string| {
      unsafe {
        let database: *notmuch_database_t = ptr::null();
        notmuch_database_open(c_string, NOTMUCH_DATABASE_MODE_READ_ONLY, ptr::to_unsafe_ptr(&database));
        Database::new(database)
      }
    }
  }

  #[fixed_stack_segment]
  pub fn tags(&self) -> Tags {
    unsafe {
      let tags = notmuch_database_get_all_tags(self.database);
      Tags::new(tags)
    }
  }

  pub fn query(&self, query: Option<~str>) -> Query {
    match query {
      Some(str) => { Query::new(self.database, str) },
      None => { Query::new(self.database, "*") },
    }
  }
}

impl Drop for Database {
  #[fixed_stack_segment]
  #[inline(never)]
  fn drop(&mut self) {
    unsafe {
      notmuch_database_close(self.database);
    }
  }
}