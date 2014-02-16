use std::ptr;
use std::run::{Process,ProcessOptions};
use std::str::from_utf8;
use cbits::notmuch::{notmuch_database_open, notmuch_database_close, notmuch_database_get_all_tags,notmuch_database_t,NOTMUCH_DATABASE_MODE_READ_ONLY};
use tags::Tags;
use query::Query;

pub struct Database {
  database: *notmuch_database_t,
}

fn get_database_path_from_cfg() -> ~str {
  let mut pr = Process::new("notmuch", [~"config", ~"get", ~"database.path"], ProcessOptions::new()).unwrap();
  let output = pr.finish_with_output();

  let utf8string = from_utf8(output.output).unwrap();
  utf8string.trim().to_owned()
}

impl Database {
  pub fn new(database: *notmuch_database_t) -> Database {
    Database { database: database }
  }

  pub fn open(path: Option<~str>) -> Database {
    let database_path = match path {
      Some(str) => { str },
      None => { get_database_path_from_cfg() }
    };
    database_path.with_c_str(|c_string| {
      unsafe {
        let database: *notmuch_database_t = ptr::null();
        notmuch_database_open(c_string, NOTMUCH_DATABASE_MODE_READ_ONLY, &database);
        Database::new(database)
      }
    })
  }

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
  fn drop(&mut self) {
    unsafe {
      notmuch_database_close(self.database);
    }
  }
}
