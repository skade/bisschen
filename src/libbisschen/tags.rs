use cbits::notmuch::{notmuch_tags_get,notmuch_tags_move_to_next,notmuch_tags_valid,notmuch_tags_t};
use std::c_str::CString;

#[deriving(Clone, Eq)]
pub struct Tag {
  str: ~str
}

pub struct Tags {
  priv pointer: *notmuch_tags_t,
  priv loaded: ~[Tag],
}

pub struct TagsIterator<'a> {
  tags: &'a mut Tags,
  index: uint,
}

impl Tags {
  pub fn new(tags: *notmuch_tags_t) -> Tags {
    Tags { pointer: tags, loaded: ~[] }
  }

  pub fn iter<'a>(&'a mut self) -> TagsIterator<'a> {
    TagsIterator { tags: self, index: 0 }
  }

  pub fn idx(&self, index: uint) -> Option<Tag> {
    let option = self.loaded.iter().idx(index);
    match option {
      Some(item) => { Some(item.clone()) },
      None => { None }
    }
  }

  fn advance_tag_pointer(&mut self) {
    unsafe {
      let tag = notmuch_tags_get(self.pointer);
      notmuch_tags_move_to_next(self.pointer);

      let c_string = CString::new(tag, false);

      let string = match c_string.as_str() {
        Some(str) => { str.to_owned() }
        None => { fail!("Tags should never yield illegal strings!") }
      };
      self.loaded.push(Tag { str: string });
    }
  }

  fn has_more(&self) -> bool {
    unsafe {
      notmuch_tags_valid(self.pointer) == 1
    }
  }

  fn get_next_tag(&mut self, index: uint) -> Option<Tag> {
    if self.has_more() {
      self.advance_tag_pointer();
      self.idx(index)
    } else {
      None
    }
  }
}

impl<'a> Iterator<Tag> for TagsIterator<'a> {
  fn next(&mut self) -> Option<Tag> {
    let idx = self.index;
    self.index += 1;
    self.tags.idx(idx).or(self.tags.get_next_tag(idx))
  }
}

#[cfg(test)]
mod test {
  use super::{Tags};
  use cbits::notmuch::{notmuch_database_open,notmuch_database_t,notmuch_database_get_all_tags,NOTMUCH_DATABASE_MODE_READ_ONLY};
  use std::ptr;
  use std::run::{Process,ProcessOptions};
  use std::str::from_utf8;

  fn get_database_path_from_cfg() -> ~str {
    let mut pr = Process::new("notmuch", [~"config", ~"get", ~"database.path"], ProcessOptions::new());
    let output = pr.unwrap().finish_with_output();

    let utf8string = from_utf8(output.output).unwrap();
    utf8string.trim().to_owned()
  }

  fn tags(database: *notmuch_database_t) -> Tags {
    unsafe {
      let tags = notmuch_database_get_all_tags(database);
      Tags::new(tags)
    }
  }

  fn load_tags_from_database() -> Tags {
    let database_path = get_database_path_from_cfg();
    let database: *notmuch_database_t = ptr::null();
    database_path.with_c_str(|c_string| {
      unsafe {
        notmuch_database_open(c_string, NOTMUCH_DATABASE_MODE_READ_ONLY, &database)
      }
    });
    tags(database)
  }

  #[test]
  fn test_load_tags_from_database() {
    load_tags_from_database();
  }

  #[test]
  fn iterate_twice() {
    let mut tags = load_tags_from_database();

    assert_eq!(tags.idx(1), None);

    for tag in tags.iter().take(2) {
      tag;
    }

    assert!(!tags.idx(1).is_none());
    assert!(tags.idx(2).is_none());

    for tag in tags.iter().take(3) {
      tag;
    }

    assert!(!tags.idx(2).is_none());
  }
}
