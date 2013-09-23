#[crate_type = "lib"];
#[link(name = "database", vers = "0.01")];

extern mod c;
extern mod extra;

use std::ptr;
use std::c_str::*;
use c::*;
use extra::time::*;

pub struct Tags {
  priv tags: *notmuch::notmuch_tags_t,
}

pub struct Query {
  priv query: *notmuch::notmuch_query_t,
  offset: Option<int>,
  limit: Option<int>,
}

pub struct Thread {
  priv thread: *notmuch::notmuch_thread_t,
}

pub struct Threads {
  priv threads: *notmuch::notmuch_threads_t,
  query: Query,
  current: int
}

pub struct Messages {
  priv messages: *notmuch::notmuch_messages_t,
}

pub struct Message {
  priv message: *notmuch::notmuch_message_t,
}

pub struct Database {
  priv database: *notmuch::notmuch_database_t,
}

impl Tags {
  pub fn new(tags: *notmuch::notmuch_tags_t) -> Tags {
    Tags { tags: tags }
  }
}

impl Iterator<CString> for Tags {
  #[fixed_stack_segment]
  fn next(&mut self) -> Option<CString> {
    unsafe {
      if notmuch::notmuch_tags_valid(self.tags) == 1 {
        let tag = notmuch::notmuch_tags_get(self.tags);
        notmuch::notmuch_tags_move_to_next(self.tags);
        Some(CString::new(tag, false))
      } else {
        None
      }
    }
  }
}

impl Threads {
  pub fn new(threads: *notmuch::notmuch_threads_t, query: Query) -> Threads {
    Threads { threads: threads, query: query, current: 0 }
  }
}

impl Iterator<Thread> for Threads {
  #[fixed_stack_segment]
  fn next(&mut self) -> Option<Thread> {
    unsafe {
      if notmuch::notmuch_threads_valid(self.threads) == 1 {
        let below_limit = match self.query.limit {
          Some(number) => { (self.current + 1) < number }
          None => { true }
        };
        if below_limit {
          let thread = notmuch::notmuch_threads_get(self.threads);
          notmuch::notmuch_threads_move_to_next(self.threads);
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
  pub fn new(messages: *notmuch::notmuch_messages_t) -> Messages {
    Messages { messages: messages }
  }
}

impl Iterator<Message> for Messages {
  #[fixed_stack_segment]
  fn next(&mut self) -> Option<Message> {
    unsafe {
      if notmuch::notmuch_messages_valid(self.messages) == 1 {
        let message = notmuch::notmuch_messages_get(self.messages);
        notmuch::notmuch_messages_move_to_next(self.messages);
        Some(Message::new(message))
      } else {
        None
      }
    }
  }
}

impl Message {
  pub fn new(message: *notmuch::notmuch_message_t) -> Message {
    Message { message: message }
  }

  #[fixed_stack_segment]
  pub fn id(&self) -> CString {
    unsafe {
      CString::new(notmuch::notmuch_message_get_message_id(self.message), false)
    }
  }

  #[fixed_stack_segment]
  pub fn thread_id(&self) -> CString {
    unsafe {
      CString::new(notmuch::notmuch_message_get_thread_id(self.message), false)
    }
  }

  #[fixed_stack_segment]
  pub fn replies(&self) -> Messages {
    unsafe {
      Messages::new(notmuch::notmuch_message_get_replies(self.message))
    }
  }

  #[fixed_stack_segment]
  pub fn header(&self, header: &str) -> CString {
    unsafe {
      do header.with_c_str
        |c_string| { CString::new(notmuch::notmuch_message_get_header(self.message, c_string), false) }
    }
  }

  pub fn subject(&self) -> CString {
    self.header("subject")
  }

  #[fixed_stack_segment]
  pub fn filename(&self) -> CString {
    unsafe {
      CString::new(notmuch::notmuch_message_get_filename(self.message),false)
    }
  }

  #[fixed_stack_segment]
  pub fn date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch::notmuch_message_get_date(self.message), 0)
    }
  }

  #[fixed_stack_segment]
  pub fn tags(&self) -> Tags {
    unsafe {
      Tags::new(notmuch::notmuch_message_get_tags(self.message))
    }
  }
}

impl Thread {
  pub fn new(thread: *notmuch::notmuch_thread_t) -> Thread {
    Thread { thread: thread }
  }

  #[fixed_stack_segment]
  pub fn id(&self) -> CString {
    unsafe {
      CString::new(notmuch::notmuch_thread_get_thread_id(self.thread), false)
    }
  }

  #[fixed_stack_segment]
  pub fn message_count(&self) -> int {
    unsafe {
      notmuch::notmuch_thread_get_total_messages(self.thread).to_int()
    }
  }

  #[fixed_stack_segment]
  pub fn subject(&self) -> CString {
    unsafe {
      CString::new(notmuch::notmuch_thread_get_subject(self.thread), false)
    }
  }

  #[fixed_stack_segment]
  pub fn authors(&self) -> CString {
    unsafe {
      CString::new(notmuch::notmuch_thread_get_authors(self.thread), false)
    }
  }

  #[fixed_stack_segment]
  pub fn oldest_message_date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch::notmuch_thread_get_oldest_date(self.thread), 0)
    }
  }

  #[fixed_stack_segment]
  pub fn newest_message_date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch::notmuch_thread_get_newest_date(self.thread), 0)
    }
  }

  #[fixed_stack_segment]
  pub fn tags(&self) -> Tags {
    unsafe {
      Tags::new(notmuch::notmuch_thread_get_tags(self.thread))
    }
  }

  #[fixed_stack_segment]
  pub fn match_messages_count(&self) -> int {
    unsafe {
      notmuch::notmuch_thread_get_matched_messages(self.thread).to_int()
    }
  }

  #[fixed_stack_segment]
  pub fn messages(&self) -> Messages {
    unsafe {
      Messages::new(notmuch::notmuch_thread_get_messages(self.thread))
    }
  }
  #[fixed_stack_segment]
  pub fn toplevel_messages(&self) -> Messages {
    unsafe {
      Messages::new(notmuch::notmuch_thread_get_toplevel_messages(self.thread))
    }
  }
}

impl Query {
  #[fixed_stack_segment]
  pub fn new(database: *notmuch::notmuch_database_t, query: &str, limit: Option<int>, offset: Option<int>) -> Query {
    unsafe {
      do query.with_c_str |c_string| {
        let query_obj = notmuch::notmuch_query_create(database, c_string);
        Query { query: query_obj, limit: limit, offset: offset }
      }
    }
  }

  #[fixed_stack_segment]
  pub fn message_count(&self) -> int {
    unsafe {
      notmuch::notmuch_query_count_messages(self.query).to_int()
    }
  }

  #[fixed_stack_segment]
  pub fn thread_count(&self) -> int {
    unsafe {
      notmuch::notmuch_query_count_threads(self.query).to_int()
    }
  }

  #[fixed_stack_segment]
  pub fn threads(self) -> Threads {
    unsafe {
      let threads = notmuch::notmuch_query_search_threads(self.query);
      Threads::new(threads, self)
    }
  }
}

impl Database {
  pub fn new(database: *notmuch::notmuch_database_t) -> Database {
    Database { database: database }
  }

  #[fixed_stack_segment]
  pub fn open(path: &str) -> Database {
    do path.with_c_str |c_string| {
      unsafe {
        let database: *notmuch::notmuch_database_t = ptr::null();
        notmuch::notmuch_database_open(c_string, notmuch::NOTMUCH_DATABASE_MODE_READ_ONLY, ptr::to_unsafe_ptr(&database));
        Database::new(database)
      }
    }
  }

  #[fixed_stack_segment]
  pub fn tags(&self) -> Tags {
    unsafe {
      let tags = notmuch::notmuch_database_get_all_tags(self.database);
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
      notmuch::notmuch_database_close(self.database);
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