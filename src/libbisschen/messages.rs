extern crate extra;

use std::c_str::CString;

use cbits::notmuch::{notmuch_messages_get,notmuch_messages_move_to_next,notmuch_messages_valid,notmuch_message_t,notmuch_message_get_message_id,notmuch_message_get_thread_id,notmuch_message_get_replies,notmuch_messages_t,notmuch_message_get_header,notmuch_message_get_filename,notmuch_message_get_date,notmuch_message_get_tags};
use extra::time::Timespec;
use tags::Tags;

#[deriving(Clone, Eq)]
pub struct Message {
  message: *notmuch_message_t,
}

pub struct Messages {
  pointer: *notmuch_messages_t,
  loaded: ~[Message],
}

pub struct MessagesIterator<'a> {
  messages: &'a mut Messages,
  index: uint,
}

impl Messages {
  pub fn new(messages: *notmuch_messages_t) -> Messages {
    Messages { pointer: messages, loaded: ~[] }
  }

  pub fn iter<'a>(&'a mut self) -> MessagesIterator<'a> {
    MessagesIterator { messages: self, index: 0 }
  }

  pub fn idx(&self, index: uint) -> Option<Message> {
    let option = self.loaded.iter().idx(index);

    match option {
      Some(item) => { Some(item.clone()) },
      None => { None }
    }
  }

  fn advance_message_pointer(&mut self) {
    unsafe {
      let message = notmuch_messages_get(self.pointer);
      notmuch_messages_move_to_next(self.pointer);

      self.loaded.push(Message { message: message });
    }
  }

  fn has_more(&self) -> bool {
    unsafe {
      notmuch_messages_valid(self.pointer) == 1
    }
  }

  fn get_next_message(&mut self) -> Option<Message> {
    if self.has_more() {
      self.advance_message_pointer();
      self.idx(self.loaded.len() - 1)
    } else {
      None
    }
  }
}

impl<'a> Iterator<Message> for MessagesIterator<'a> {
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

  pub fn id(&self) -> ~str {
    unsafe {
      let c_string = CString::new(notmuch_message_get_message_id(self.message), false);
      c_string.as_str().unwrap().to_owned()
    }
  }

  pub fn thread_id(&self) -> CString {
    unsafe {
      CString::new(notmuch_message_get_thread_id(self.message), false)
    }
  }

  pub fn replies(&self) -> Messages {
    unsafe {
      Messages::new(notmuch_message_get_replies(self.message))
    }
  }

  pub fn header(&self, header: &str) -> CString {
    unsafe {
      header.with_c_str(|c_string| {
        CString::new(notmuch_message_get_header(self.message, c_string), false)
      })
    }
  }

  pub fn subject(&self) -> ~str {
    let c_string = self.header("subject");
    c_string.as_str().unwrap().to_owned()
  }

  pub fn filename(&self) -> ~str {
    unsafe {
      let c_string =  CString::new(notmuch_message_get_filename(self.message),false);
      c_string.as_str().unwrap().to_owned()
    }
  }

  pub fn date(&self) -> Timespec {
    unsafe {
      Timespec::new(notmuch_message_get_date(self.message), 0)
    }
  }

  pub fn tags(&self) -> Tags {
    unsafe {
      Tags::new(notmuch_message_get_tags(self.message))
    }
  }
}

#[cfg(test)]
mod test {
  use super::Messages;
  use cbits::notmuch::{notmuch_database_open,notmuch_database_t,notmuch_query_create,notmuch_query_search_messages,NOTMUCH_DATABASE_MODE_READ_ONLY};
  use std::ptr;
  use std::run::{Process,ProcessOptions};
  use std::str::from_utf8;

  fn get_database_path_from_cfg() -> ~str {
    let mut pr = Process::new("notmuch", [~"config", ~"get", ~"database.path"], ProcessOptions::new());
    let output = pr.unwrap().finish_with_output();

    let utf8string = from_utf8(output.output).unwrap();
    utf8string.trim().to_owned()
  }

  fn messages(database: *notmuch_database_t) -> Messages {
    unsafe {
      "*".with_c_str(|c_string| {
        let query = notmuch_query_create(database, c_string);
        let messages = notmuch_query_search_messages(query);
        Messages::new(messages)
      })
    }
  }

  fn load_messages_from_database() -> Messages {
    let database_path = get_database_path_from_cfg();
    let database: *notmuch_database_t = ptr::null();
    database_path.with_c_str(|c_string| {
      unsafe {
        notmuch_database_open(c_string, NOTMUCH_DATABASE_MODE_READ_ONLY, &database)
      }
    });
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
      message;
    }

    assert!(!messages.idx(2).is_none());

    for message in messages.iter().take(30) {
      message;
    }
  }
}
