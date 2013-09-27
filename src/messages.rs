extern mod extra;

use std::c_str::*;
use c::notmuch::*;
use extra::time::*;
use tags::*;

pub struct Messages {
  priv messages: *notmuch_messages_t,
}

pub struct Message {
  priv message: *notmuch_message_t,
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