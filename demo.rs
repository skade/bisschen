extern mod c;
extern mod database;

use c::*;
use database::*;
use std::comm::*;
use std::c_str::*;
use std::util::*;

struct List<'self> {
  contents: &'self Lines,
}

trait Lines {
  fn lines<'a>(&'a self) -> &'a Iterator<CString>;
}

impl Lines for Tags {
  fn lines<'a>(&'a self) -> &'a Iterator<CString> {
    self as &'a Iterator<CString>
  }
}

impl<'self> List<'self> {
  fn new(contents: &'self Lines) -> List<'self> {
    List { contents: contents }
  }

  fn print_lines(&mut self) {
    let mut contents = self.contents;
    for line in contents.lines() {
      match line.as_str() {
        Some(str) => { println(str) }
        None => { }
      }
    }
  }
}

fn main() {
  let database = Database::open("/Users/skade/Mail");
  let tags: &Lines = &database.tags() as &Lines;
  let mut list = List::new(tags);
  list.print_lines()
}