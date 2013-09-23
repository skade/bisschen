extern mod c;
extern mod database;

use c::*;
use database::*;
use std::comm::*;
use std::c_str::*;
use std::util::*;

struct List<T> {
  contents: T,
}

trait Lines {
  fn lines<'a>(&'a self) -> &'a Iterator<CString>;
}

impl Lines for Tags {
  fn lines<'a>(&'a self) -> &'a Iterator<CString> {
    self as &Iterator<CString>
  }
}

impl<T: Lines> List<T> {
  fn new(contents: T) -> List<T> {
    List { contents: contents }
  }

  fn print_lines(&mut self) {
    let mut lines = self.contents.lines();
    match lines.next() {
      Some(c_string) => { }
      None => { }
    }

    //for line in self.contents.lines() {
    //  match line.as_str() {
    //    Some(str) => { println(str) }
    //    None => { }
    //  }
    //}
  }
}

fn main() {
  let database = Database::open("/Users/skade/Mail");
  let tags = database.tags();
  let mut list: List<Tags> = List::new(tags);
  list.print_lines()
}