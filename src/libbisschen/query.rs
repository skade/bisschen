use threads::Threads;
use messages::Messages;
use cbits::notmuch::{notmuch_query_create,notmuch_query_count_messages,notmuch_query_search_threads,notmuch_query_count_threads,notmuch_query_t,notmuch_database_t,notmuch_query_search_messages};

pub struct Query {
  priv query: *notmuch_query_t,
}

impl Query {
  pub fn new(database: *notmuch_database_t, query: &str) -> Query {
    unsafe {
      query.with_c_str(|c_string| {
        let query_obj = notmuch_query_create(database, c_string);
        Query { query: query_obj }
      })
    }
  }

  pub fn message_count(&self) -> int {
    unsafe {
      notmuch_query_count_messages(self.query).to_int().unwrap()
    }
  }

  pub fn thread_count(&self) -> int {
    unsafe {
      notmuch_query_count_threads(self.query).to_int().unwrap()
    }
  }

  pub fn threads(self) -> Threads {
    unsafe {
      let threads = notmuch_query_search_threads(self.query);
      Threads::new(threads)
    }
  }

  pub fn messages(self) -> Messages {
    unsafe {
      let messages = notmuch_query_search_messages(self.query);
      Messages::new(messages)
    }
  }
}

#[cfg(test)]
mod tests {
  use database::Database;

  #[test]
  fn print_message_count() {
    let database = Database::open(None);
    let query = database.query(None);
    let count = query.message_count();
    assert!(count > 0);
  }

  #[test]
  fn print_thread_count() {
    let database = Database::open(None);
    let query = database.query(None);
    let count = query.thread_count();
    assert!(count > 0);
  }
}
