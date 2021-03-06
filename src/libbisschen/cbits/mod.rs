pub mod notmuch {
  use std::libc::{c_char,c_int,time_t};

  pub struct notmuch_config_t;
  pub struct notmuch_tags_t;
  pub struct notmuch_database_t;
  pub struct notmuch_status_t;
  pub struct notmuch_query_t;
  pub struct notmuch_threads_t;
  pub struct notmuch_thread_t;
  pub struct notmuch_messages_t;
  pub struct notmuch_message_t;
  pub struct notmuch_filenames_t;

  pub enum notmuch_database_mode_t {
      NOTMUCH_DATABASE_MODE_READ_ONLY = 0,
      NOTMUCH_DATABASE_MODE_READ_WRITE = 1
  }

  extern {
    pub fn notmuch_database_open(path: *c_char, mode: notmuch_database_mode_t, notmuch_database_t: **notmuch_database_t) -> notmuch_status_t;
    pub fn notmuch_database_close(database: *notmuch_database_t);
    pub fn notmuch_database_get_all_tags(database: *notmuch_database_t) -> *notmuch_tags_t;
    pub fn notmuch_tags_get(tags: *notmuch_tags_t) -> *c_char;
    pub fn notmuch_tags_valid(tags: *notmuch_tags_t) -> c_int;
    pub fn notmuch_tags_move_to_next(tags: *notmuch_tags_t);
    pub fn notmuch_query_create(database: *notmuch_database_t, query_string: *c_char) -> *notmuch_query_t;
    pub fn notmuch_query_count_messages(query: *notmuch_query_t) -> c_int;
    pub fn notmuch_query_count_threads(query: *notmuch_query_t) -> c_int;
    pub fn notmuch_query_search_threads(query: *notmuch_query_t) -> *notmuch_threads_t;
    pub fn notmuch_query_search_messages(query: *notmuch_query_t) -> *notmuch_messages_t;
    pub fn notmuch_threads_get(threads: *notmuch_threads_t) -> *notmuch_thread_t;
    pub fn notmuch_threads_valid(threads: *notmuch_threads_t) -> c_int;
    pub fn notmuch_threads_move_to_next(threads: *notmuch_threads_t);
    pub fn notmuch_thread_get_total_messages(thread: *notmuch_thread_t) -> c_int;
    pub fn notmuch_thread_get_subject(thread: *notmuch_thread_t) -> *c_char;
    pub fn notmuch_thread_get_authors(thread: *notmuch_thread_t) -> *c_char;
    pub fn notmuch_thread_get_oldest_date(thread: *notmuch_thread_t) -> time_t;
    pub fn notmuch_thread_get_newest_date(thread: *notmuch_thread_t) -> time_t;
    pub fn notmuch_thread_get_tags(thread: *notmuch_thread_t) -> *notmuch_tags_t;
    pub fn notmuch_thread_get_matched_messages(thread: *notmuch_thread_t) -> c_int;
    pub fn notmuch_thread_get_messages(thread: *notmuch_thread_t) -> *notmuch_messages_t;
    pub fn notmuch_thread_get_toplevel_messages(thread: *notmuch_thread_t) -> *notmuch_messages_t;
    pub fn notmuch_thread_get_thread_id(thread: *notmuch_thread_t) -> *c_char;
    pub fn notmuch_messages_get(messages: *notmuch_messages_t) -> *notmuch_message_t;
    pub fn notmuch_messages_valid(messages: *notmuch_messages_t) -> c_int;
    pub fn notmuch_messages_move_to_next(messages: *notmuch_messages_t);
    pub fn notmuch_message_get_message_id(message: *notmuch_message_t) -> *c_char;
    pub fn notmuch_message_get_thread_id(message: *notmuch_message_t) -> *c_char;
    pub fn notmuch_message_get_replies(message: *notmuch_message_t) -> *notmuch_messages_t;
    pub fn notmuch_message_get_filename(message: *notmuch_message_t) -> *c_char;
    pub fn notmuch_message_get_filenames(message: *notmuch_message_t) -> *notmuch_filenames_t;
    pub fn notmuch_message_get_tags(message: *notmuch_message_t) -> *notmuch_tags_t;
    pub fn notmuch_message_get_header(message: *notmuch_message_t, header: *c_char) -> *c_char;
    pub fn notmuch_message_get_date(message: *notmuch_message_t) -> time_t;
  }
}