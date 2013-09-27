extern mod extra;

use c::notmuch::*;
use std::c_str::*;
use std::str::*;
use messages::*;
use extra::time::*;
use tags::*;

#[deriving(Clone, Eq)]
pub struct Thread {
  priv thread: *notmuch_thread_t,
}

pub struct Threads {
  priv pointer: *notmuch_threads_t,
  priv loaded: ~[Thread],
}

pub struct ThreadsIterator<'self> {
  tags: &'self mut Threads,
  index: uint,
}

impl<'self> Iterator<Thread> for ThreadsIterator<'self> {
  fn next(&mut self) -> Option<Thread> {
    let idx = self.index;
    self.index += 1;
    self.tags.idx(idx).or(self.tags.get_next_thread(idx))
  }
}

impl Threads {
  pub fn new(threads: *notmuch_threads_t) -> Threads {
    Threads { pointer: threads, loaded: ~[]}
  }

  pub fn iter<'a>(&'a mut self) -> ThreadsIterator<'a> {
    ThreadsIterator { tags: self, index: 0 }
  }

  fn idx(&self, index: uint) -> Option<Thread> {
    let option = self.loaded.iter().idx(index);
    match option {
      Some(item) => { Some(item.clone()) },
      None => { None }
    }
  }

  #[fixed_stack_segment]
  fn advance_thread_pointer(&mut self) {
    unsafe {
      let thread = notmuch_threads_get(self.pointer);
      notmuch_threads_move_to_next(self.pointer);

      self.loaded.push(Thread { thread: thread });
    }
  }

  #[fixed_stack_segment]
  fn has_more(&self) -> bool {
    unsafe {
      notmuch_threads_valid(self.pointer) == 1
    }
  }

  fn get_next_thread(&mut self, index: uint) -> Option<Thread> {
    if self.has_more() {
      self.advance_thread_pointer();
      self.idx(index)
    } else {
      None
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

#[cfg(test)]
mod test {
  use super::*;
  use c::notmuch::*;
  use std::ptr;
  use std::c_str::*;
  use std::run::*;
  use std::str::*;
  use std::util::id;

  fn get_database_path_from_cfg() -> ~str {
    let mut pr = Process::new("notmuch", [~"config", ~"get", ~"database.path"], ProcessOptions::new());
    let output = pr.finish_with_output();

    let utf8string = from_utf8(output.output);
    utf8string.trim().to_owned()
  }

  #[fixed_stack_segment]
  fn threads(database: *notmuch_database_t) -> Threads {
    unsafe {
      do "*".with_c_str |c_string| {
        let query = notmuch_query_create(database, c_string);
        let threads = notmuch_query_search_threads(query);
        Threads::new(threads)
      }
    }
  }

  #[fixed_stack_segment]
  fn load_threads_from_database() -> Threads {
    let database_path = get_database_path_from_cfg();
    let database: *notmuch_database_t = ptr::null();
    do database_path.with_c_str |c_string| {
      unsafe {
        notmuch_database_open(c_string, NOTMUCH_DATABASE_MODE_READ_ONLY, ptr::to_unsafe_ptr(&database))
      }
    };
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
