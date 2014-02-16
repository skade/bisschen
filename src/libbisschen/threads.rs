extern crate extra;

use cbits::notmuch::{notmuch_threads_get, notmuch_threads_move_to_next, notmuch_threads_valid, notmuch_thread_get_thread_id, notmuch_thread_get_messages, notmuch_thread_get_subject, notmuch_thread_get_total_messages, notmuch_thread_get_authors,notmuch_thread_get_oldest_date,notmuch_threads_t,notmuch_thread_t,notmuch_thread_get_newest_date,notmuch_thread_get_tags,notmuch_thread_get_matched_messages,notmuch_thread_get_toplevel_messages};
use std::c_str::CString;
use messages::Messages;
use tags::Tags;
use extra::time::Timespec;

#[deriving(Clone, Eq)]
pub struct Thread {
  priv thread: *notmuch_thread_t,
}

pub struct Threads {
  priv pointer: *notmuch_threads_t,
  priv loaded: ~[Thread],
}

pub struct ThreadsIterator<'a> {
  tags: &'a mut Threads,
  index: uint,
}

impl<'a> Iterator<Thread> for ThreadsIterator<'a> {
  fn next(&mut self) -> Option<Thread> {
    let idx = self.index;
    self.index += 1;
    self.tags.idx(idx).or(self.tags.get_next_thread())
  }
}

impl Threads {
  pub fn new(threads: *notmuch_threads_t) -> Threads {
    Threads { pointer: threads, loaded: ~[]}
  }

  pub fn iter<'a>(&'a mut self) -> ThreadsIterator<'a> {
    ThreadsIterator { tags: self, index: 0 }
  }

  pub fn idx(&self, index: uint) -> Option<Thread> {
    let option = self.loaded.iter().idx(index);
    match option {
      Some(item) => { Some(item.clone()) },
      None => { None }
    }
  }

  fn advance_thread_pointer(&mut self) {
    unsafe {
      let thread = notmuch_threads_get(self.pointer);
      notmuch_threads_move_to_next(self.pointer);

      self.loaded.push(Thread { thread: thread });
    }
  }

  fn has_more(&self) -> bool {
    unsafe {
      notmuch_threads_valid(self.pointer) == 1
    }
  }

  pub fn get_next_thread(&mut self) -> Option<Thread> {
    if self.has_more() {
      self.advance_thread_pointer();
      self.idx(self.loaded.len() - 1)
    } else {
      None
    }
  }
}

impl Thread {
  pub fn new(thread: *notmuch_thread_t) -> Thread {
    Thread { thread: thread }
  }

  pub fn id(&self) -> ~str {
    unsafe {
      let c_string = CString::new(notmuch_thread_get_thread_id(self.thread), false);
      c_string.as_str().unwrap().to_owned()
    }
  }

  pub fn message_count(&self) -> int {
    unsafe {
      notmuch_thread_get_total_messages(self.thread).to_int().unwrap()
    }
  }

  pub fn subject(&self) -> ~str {
    unsafe {
      let c_string = CString::new(notmuch_thread_get_subject(self.thread), false);
      c_string.as_str().unwrap().to_owned()
    }
  }

  pub fn authors(&self) -> CString {
    unsafe {
      CString::new(notmuch_thread_get_authors(self.thread), false)
    }
  }

  pub fn oldest_message_date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch_thread_get_oldest_date(self.thread), 0)
    }
  }

  pub fn newest_message_date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch_thread_get_newest_date(self.thread), 0)
    }
  }

  pub fn tags(&self) -> Tags {
    unsafe {
      Tags::new(notmuch_thread_get_tags(self.thread))
    }
  }

  pub fn match_messages_count(&self) -> int {
    unsafe {
      notmuch_thread_get_matched_messages(self.thread).to_int().unwrap()
    }
  }

  pub fn messages(&self) -> Messages {
    unsafe {
      Messages::new(notmuch_thread_get_messages(self.thread))
    }
  }

  pub fn toplevel_messages(&self) -> Messages {
    unsafe {
      Messages::new(notmuch_thread_get_toplevel_messages(self.thread))
    }
  }
}

#[cfg(test)]
mod test {
  use super::Threads;
  use cbits::notmuch::{notmuch_database_open,notmuch_database_t,notmuch_query_create,notmuch_query_search_threads,NOTMUCH_DATABASE_MODE_READ_ONLY};
  use std::ptr;
  use std::run::{Process,ProcessOptions};
  use std::str::from_utf8;
  use std::util::id;

  fn get_database_path_from_cfg() -> ~str {
    let mut pr = Process::new("notmuch", [~"config", ~"get", ~"database.path"], ProcessOptions::new());
    let output = pr.finish_with_output();

    let utf8string = from_utf8(output.output);
    utf8string.trim().to_owned()
  }

  fn threads(database: *notmuch_database_t) -> Threads {
    unsafe {
      "*".with_c_str(|c_string| {
        let query = notmuch_query_create(database, c_string);
        let threads = notmuch_query_search_threads(query);
        Threads::new(threads)
      })
    }
  }

  fn load_threads_from_database() -> Threads {
    let database_path = get_database_path_from_cfg();
    let database: *notmuch_database_t = ptr::null();
    database_path.with_c_str(|c_string| {
      unsafe {
        notmuch_database_open(c_string, NOTMUCH_DATABASE_MODE_READ_ONLY, ptr::to_unsafe_ptr(&database))
      }
    });
    threads(database)
  }

  #[test]
  fn test_load_threads_from_database() {
    load_threads_from_database();
  }

  #[test]
  fn iterate_twice() {
    let mut threads = load_threads_from_database();

    assert_eq!(threads.idx(2), None);

    for thread in threads.iter().take(20) {
      id(thread);
    }

    assert!(!threads.idx(2).is_none());

    for thread in threads.iter().take(30) {
      id(thread);
    }
  }
}
