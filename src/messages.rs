extern mod extra;

use std::c_str::*;
use std::uint::*;

use c::notmuch::*;
use extra::time::*;
use tags::*;

#[deriving(Clone, Eq)]
struct Message {
  priv message: *notmuch_message_t,
}

struct Messages {
  priv pointer: *notmuch_messages_t,
  priv loaded: ~[Message],
}

pub struct MessagesIterator<'self> {
  messages: &'self mut Messages,
  index: uint,
}

impl Messages {
  pub fn new(messages: *notmuch_messages_t) -> Messages {
    Messages { pointer: messages, loaded: ~[] }
  }

  pub fn iter<'a>(&'a mut self) -> MessagesIterator<'a> {
    MessagesIterator { messages: self, index: 0 }
  }

  fn idx(&self, index: uint) -> Option<Message> {
    let option = self.loaded.iter().idx(index);
    match option {
      Some(item) => { Some(item.clone()) },
      None => { None }
    }
  }

  #[fixed_stack_segment]
  fn advance_message_pointer(&mut self) {
    unsafe {
      let message = notmuch_messages_get(self.pointer);
      notmuch_messages_move_to_next(self.pointer);

      self.loaded.push(Message { message: message });
    }
  }

  #[fixed_stack_segment]
  fn has_more(&self) -> bool {
    unsafe {
      notmuch_messages_valid(self.pointer) == 1
    }
  }

  fn get_next_message(&mut self) -> Option<Message> {
    if self.has_more() {
      self.advance_message_pointer();
      self.idx(self.loaded.len())
    } else {
      None
    }
  }
}

impl<'self> Iterator<Message> for MessagesIterator<'self> {
  fn next(&mut self) -> Option<Message> {
    let idx = self.index;
    self.index += 1;
    self.messages.idx(idx).or(self.messages.get_next_message())
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
  fn messages(database: *notmuch_database_t) -> Messages {
    unsafe {
      do "*".with_c_str |c_string| {
        let query = notmuch_query_create(database, c_string);
        let messages = notmuch_query_search_messages(query);
        Messages::new(messages)
      }
    }
  }

  #[fixed_stack_segment]
  fn load_messages_from_database() -> Messages {
    let database_path = get_database_path_from_cfg();
    let database: *notmuch_database_t = ptr::null();
    do database_path.with_c_str |c_string| {
      unsafe {
        notmuch_database_open(c_string, NOTMUCH_DATABASE_MODE_READ_ONLY, ptr::to_unsafe_ptr(&database))
      }
    };
    messages(database)
  }

  #[test]
  fn test_load_messages_from_database() {
    load_messages_from_database();
  }

  #[test]
  fn iterate_twice() {
    let mut messages = load_messages_from_database();

    assert_eq!(messages.idx(2), None);

    for message in messages.iter().take(20) {
      id(message);
    }

    assert!(!messages.idx(2).is_none());

    for message in messages.iter().take(30) {
      id(message);
    }
  }
}
