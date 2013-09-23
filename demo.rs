extern mod c;
extern mod database;

use database::*;
use std::c_str::*;

struct List<T> {
  contents: T,
}

trait Lines {
  fn lines<'a>(&'a mut self) -> &'a mut Iterator<CString>;
}

impl Lines for Tags {
  fn lines<'a>(&'a mut self) -> &'a mut Iterator<CString> {
    self as &mut Iterator<CString>
  }
}

impl<T: Lines> List<T> {
  fn new(contents: T) -> List<T> {
    List { contents: contents }
  }

  fn print_lines(&mut self) {
    let mut lines = self.contents.lines();
    for line in lines {
      match line.as_str() {
        Some(str) => { println(str) }
        None => { }
      }
    }
  }
}

fn main() {
  let database = Database::open("/Users/skade/Mail");
  let tags = database.tags();
  let mut list: List<Tags> = List::new(tags);
  list.print_lines()
}