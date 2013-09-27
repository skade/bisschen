use std::ptr;
use std::c_str::*;
use std::run::*;
use std::str::*;
use c::notmuch::*;
use extra::time::*;
use tags::*;

pub struct Query {
  priv query: *notmuch_query_t,
  offset: Option<int>,
  limit: Option<int>,
}

pub struct Thread {
  priv thread: *notmuch_thread_t,
}

pub struct Threads {
  priv threads: *notmuch_threads_t,
  query: Query,
  current: int
}

pub struct Messages {
  priv messages: *notmuch_messages_t,
}

pub struct Message {
  priv message: *notmuch_message_t,
}

pub struct Database {
  priv database: *notmuch_database_t,
}

impl Threads {
  pub fn new(threads: *notmuch_threads_t, query: Query) -> Threads {
    Threads { threads: threads, query: query, current: 0 }
  }
}

impl Iterator<Thread> for Threads {
  #[fixed_stack_segment]
  fn next(&mut self) -> Option<Thread> {
    unsafe {
      if notmuch_threads_valid(self.threads) == 1 {
        let below_limit = match self.query.limit {
          Some(number) => { (self.current + 1) < number }
          None => { true }
        };
        if below_limit {
          let thread = notmuch_threads_get(self.threads);
          notmuch_threads_move_to_next(self.threads);
          self.current += 1;
          Some(Thread::new(thread))
        } else {
          None
        }
      } else {
        None
      }
    }
  }
}

impl Messages {
  pub fn new(messages: *notmuch_messages_t) -> Messages {
    Messages { messages: messages }
  }
}

impl Iterator<Message> for Messages {
  #[fixed_stack_segment]
  fn next(&mut self) -> Option<Message> {
    unsafe {
      if notmuch_messages_valid(self.messages) == 1 {
        let message = notmuch_messages_get(self.messages);
        notmuch_messages_move_to_next(self.messages);
        Some(Message::new(message))
      } else {
        None
      }
    }
  }
}

impl Message {
  pub fn new(message: *notmuch_message_t) -> Message {
    Message { message: message }
  }

  #[fixed_stack_segment]
  pub fn id(&self) -> CString {
    unsafe {
      CString::new(notmuch_message_get_message_id(self.message), false)
    }
  }

  #[fixed_stack_segment]
  pub fn thread_id(&self) -> CString {
    unsafe {
      CString::new(notmuch_message_get_thread_id(self.message), false)
    }
  }

  #[fixed_stack_segment]
  pub fn replies(&self) -> Messages {
    unsafe {
      Messages::new(notmuch_message_get_replies(self.message))
    }
  }

  #[fixed_stack_segment]
  pub fn header(&self, header: &str) -> CString {
    unsafe {
      do header.with_c_str
        |c_string| { CString::new(notmuch_message_get_header(self.message, c_string), false) }
    }
  }

  pub fn subject(&self) -> CString {
    self.header("subject")
  }

  #[fixed_stack_segment]
  pub fn filename(&self) -> CString {
    unsafe {
      CString::new(notmuch_message_get_filename(self.message),false)
    }
  }

  #[fixed_stack_segment]
  pub fn date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch_message_get_date(self.message), 0)
    }
  }

  #[fixed_stack_segment]
  pub fn tags(&self) -> Tags {
    unsafe {
      Tags::new(notmuch_message_get_tags(self.message))
    }
  }
}

impl Thread {
  pub fn new(thread: *notmuch_thread_t) -> Thread {
    Thread { thread: thread }
  }

  #[fixed_stack_segment]
  pub fn id(&self) -> CString {
    unsafe {
      CString::new(notmuch_thread_get_thread_id(self.thread), false)
    }
  }

  #[fixed_stack_segment]
  pub fn message_count(&self) -> int {
    unsafe {
      notmuch_thread_get_total_messages(self.thread).to_int()
    }
  }

  #[fixed_stack_segment]
  pub fn subject(&self) -> CString {
    unsafe {
      CString::new(notmuch_thread_get_subject(self.thread), false)
    }
  }

  #[fixed_stack_segment]
  pub fn authors(&self) -> CString {
    unsafe {
      CString::new(notmuch_thread_get_authors(self.thread), false)
    }
  }

  #[fixed_stack_segment]
  pub fn oldest_message_date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch_thread_get_oldest_date(self.thread), 0)
    }
  }

  #[fixed_stack_segment]
  pub fn newest_message_date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch_thread_get_newest_date(self.thread), 0)
    }
  }

  #[fixed_stack_segment]
  pub fn tags(&self) -> Tags {
    unsafe {
      Tags::new(notmuch_thread_get_tags(self.thread))
    }
  }

  #[fixed_stack_segment]
  pub fn match_messages_count(&self) -> int {
    unsafe {
      notmuch_thread_get_matched_messages(self.thread).to_int()
    }
  }

  #[fixed_stack_segment]
  pub fn messages(&self) -> Messages {
    unsafe {
      Messages::new(notmuch_thread_get_messages(self.thread))
    }
  }
  #[fixed_stack_segment]
  pub fn toplevel_messages(&self) -> Messages {
    unsafe {
      Messages::new(notmuch_thread_get_toplevel_messages(self.thread))
    }
  }
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
      Threads::new(threads, self)
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

#[test]
fn print_tags() {
  let database = Database::open("/Users/skade/Mail");
  for tag in database.tags() {
    match tag.as_str() {
      Some(str) => { println(str) },
      None => { }
    }
  }
}

#[test]
fn print_threads() {
  let database = Database::open("/Users/skade/Mail");
  let mut threads = database.query("*", Some(20), Some(0)).threads();
  for thread in threads {
    println(thread.message_count().to_str());
    let subject = thread.subject();
    match subject.as_str() {
      Some(str) => { println(str) },
      None => { }
    }
    let authors = thread.authors();
    match authors.as_str() {
      Some(str) => { println(str) },
      None => { }
    }
    let oldest_date = thread.oldest_message_date();
    let local = at(oldest_date);
    println(local.strftime("%F"));
    for tag in thread.tags() {
      match tag.as_str() {
        Some(str) => { println(str) },
        None => { }
      }
    }
  }
}

#[test]
fn print_message_count() {
  let database = Database::open("/Users/skade/Mail");
  let query = database.query("*", None, None);
  let count = query.message_count();
  println(count.to_str());
}

#[test]
fn print_thread_count() {
  let database = Database::open("/Users/skade/Mail");
  let query = database.query("*", None, None);
  let count = query.thread_count();
  println(count.to_str());
}