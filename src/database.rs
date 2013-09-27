use std::ptr;
use std::c_str::*;
use std::run::*;
use std::str::*;
use c::notmuch::*;
use tags::*;
use threads::*;

pub struct Query {
  priv query: *notmuch_query_t,
  offset: Option<int>,
  limit: Option<int>,
}

pub struct Database {
  priv database: *notmuch_database_t,
}

impl Query {
  #[fixed_stack_segment]
  pub fn new(database: *notmuch_database_t, query: &str, limit: Option<int>, offset: Option<int>) -> Query {
    unsafe {
      do query.with_c_str |c_string| {
        let query_obj = notmuch_query_create(database, c_string);
        Query { query: query_obj, limit: limit, offset: offset }
      }
    }
  }

  #[fixed_stack_segment]
  pub fn message_count(&self) -> int {
    unsafe {
      notmuch_query_count_messages(self.query).to_int()
    }
  }

  #[fixed_stack_segment]
  pub fn thread_count(&self) -> int {
    unsafe {
      notmuch_query_count_threads(self.query).to_int()
    }
  }

  #[fixed_stack_segment]
  pub fn threads(self) -> Threads {
    unsafe {
      let threads = notmuch_query_search_threads(self.query);
      Threads::new(threads)
    }
  }
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

  pub fn query(&self, query: &str, offset: Option<int>, limit: Option<int>) -> Query {
    Query::new(self.database, query, offset, limit)
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

//#[test]
//fn print_tags() {
//  let database = Database::open("/Users/skade/Mail");
//  for tag in database.tags() {
//    match tag.as_str() {
//      Some(str) => { println(str) },
//      None => { }
//    }
//  }
//}
//
//#[test]
//fn print_threads() {
//  let database = Database::open("/Users/skade/Mail");
//  let mut threads = database.query("*", Some(20), Some(0)).threads();
//  for thread in threads {
//    println(thread.message_count().to_str());
//    let subject = thread.subject();
//    match subject.as_str() {
//      Some(str) => { println(str) },
//      None => { }
//    }
//    let authors = thread.authors();
//    match authors.as_str() {
//      Some(str) => { println(str) },
//      None => { }
//    }
//    let oldest_date = thread.oldest_message_date();
//    let local = at(oldest_date);
//    println(local.strftime("%F"));
//    for tag in thread.tags() {
//      match tag.as_str() {
//        Some(str) => { println(str) },
//        None => { }
//      }
//    }
//  }
//}
//
//#[test]
//fn print_message_count() {
//  let database = Database::open("/Users/skade/Mail");
//  let query = database.query("*", None, None);
//  let count = query.message_count();
//  println(count.to_str());
//}
//
//#[test]
//fn print_thread_count() {
//  let database = Database::open("/Users/skade/Mail");
//  let query = database.query("*", None, None);
//  let count = query.thread_count();
//  println(count.to_str());
//}