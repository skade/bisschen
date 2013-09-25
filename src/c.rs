mod notmuch {
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
    fn notmuch_database_open(path: *c_char, mode: notmuch_database_mode_t, notmuch_database_t: **notmuch_database_t) -> notmuch_status_t;
    fn notmuch_database_close(database: *notmuch_database_t);
    fn notmuch_database_get_all_tags(database: *notmuch_database_t) -> *notmuch_tags_t;
    fn notmuch_tags_get(tags: *notmuch_tags_t) -> *c_char;
    fn notmuch_tags_valid(tags: *notmuch_tags_t) -> c_int;
    fn notmuch_tags_move_to_next(tags: *notmuch_tags_t);
    fn notmuch_query_create(database: *notmuch_database_t, query_string: *c_char) -> *notmuch_query_t;
    fn notmuch_query_count_messages(query: *notmuch_query_t) -> c_int;
    fn notmuch_query_count_threads(query: *notmuch_query_t) -> c_int;
    fn notmuch_query_search_threads(query: *notmuch_query_t) -> *notmuch_threads_t;
    fn notmuch_threads_get(threads: *notmuch_threads_t) -> *notmuch_thread_t;
    fn notmuch_threads_valid(threads: *notmuch_threads_t) -> c_int;
    fn notmuch_threads_move_to_next(threads: *notmuch_threads_t);
    fn notmuch_thread_get_total_messages(thread: *notmuch_thread_t) -> c_int;
    fn notmuch_thread_get_subject(thread: *notmuch_thread_t) -> *c_char;
    fn notmuch_thread_get_authors(thread: *notmuch_thread_t) -> *c_char;
    fn notmuch_thread_get_oldest_date(thread: *notmuch_thread_t) -> time_t;
    fn notmuch_thread_get_newest_date(thread: *notmuch_thread_t) -> time_t;
    fn notmuch_thread_get_tags(thread: *notmuch_thread_t) -> *notmuch_tags_t;
    fn notmuch_thread_get_matched_messages(thread: *notmuch_thread_t) -> c_int;
    fn notmuch_thread_get_messages(thread: *notmuch_thread_t) -> *notmuch_messages_t;
    fn notmuch_thread_get_toplevel_messages(thread: *notmuch_thread_t) -> *notmuch_messages_t;
    fn notmuch_thread_get_thread_id(thread: *notmuch_thread_t) -> *c_char;
    fn notmuch_messages_get(messages: *notmuch_messages_t) -> *notmuch_message_t;
    fn notmuch_messages_valid(messages: *notmuch_messages_t) -> c_int;
    fn notmuch_messages_move_to_next(messages: *notmuch_messages_t);
    fn notmuch_message_get_message_id(message: *notmuch_message_t) -> *c_char;
    fn notmuch_message_get_thread_id(message: *notmuch_message_t) -> *c_char;
    fn notmuch_message_get_replies(message: *notmuch_message_t) -> *notmuch_messages_t;
    fn notmuch_message_get_filename(message: *notmuch_message_t) -> *c_char;
    fn notmuch_message_get_filenames(message: *notmuch_message_t) -> *notmuch_filenames_t;
    fn notmuch_message_get_tags(message: *notmuch_message_t) -> *notmuch_tags_t;
    fn notmuch_message_get_header(message: *notmuch_message_t, header: *c_char) -> *c_char;
    fn notmuch_message_get_date(message: *notmuch_message_t) -> time_t;
  }
}

mod ncurses {
  use std::libc::{c_char, c_short, c_int, c_uint};

  struct WINDOW;
  struct SCREEN;

  pub type chtype = c_uint;
  pub type attr_t = c_int;
  pub type NCURSES_ATTR_T = attr_t;

  pub static COLOR_BLACK   : c_short = 0;
  pub static COLOR_RED     : c_short = 1;
  pub static COLOR_GREEN   : c_short = 2;
  pub static COLOR_YELLOW  : c_short = 3;
  pub static COLOR_BLUE    : c_short = 4;
  pub static COLOR_MAGENTA : c_short = 5;
  pub static COLOR_CYAN    : c_short = 6;
  pub static COLOR_WHITE   : c_short = 7;

  extern {
    fn initscr() -> *WINDOW;
    fn getch() -> c_int;
    fn clear() -> c_int;
    fn refresh() -> c_int;
    fn endwin() -> c_int;
    fn noecho() -> c_int;
    fn printw(characters: *c_char) -> c_int;
    fn move(x: c_int, y: c_int) -> c_int;
    fn start_color() ->  c_int;
    fn init_pair(pair: c_short, foreground: c_short, background: c_short) -> c_int;
  }
}

mod termbox {
  use std::libc::c_int;

  pub struct tb_cell {
    character: u32,
    foreground: u16,
    background: u16,
  }

  pub struct tb_event {
    event_type: u8,
    modifier: u8, 
    key: u16,
    ch: u32,
    w: i32,
    h: i32,
  }

  enum event_type {
    TB_EVENT_KEY = 1,
    TB_EVENT_RESIZE = 2,
  }

  enum input_mode {
    TB_INPUT_CURRENT = 0,
    TB_INPUT_ESC = 1,
    TB_INPUT_ALT = 2,
  }

  extern {
    fn tb_init() -> c_int;
    fn tb_shutdown();

    fn tb_width() -> c_int;
    fn tb_height() -> c_int;

    fn tb_clear();
    fn tb_set_clear_attributes(fg: u16, bg: u16);

    fn tb_present();

    fn tb_set_cursor(cx: c_int, cy: c_int);

    fn tb_put_cell(x: c_int, y: c_int, cell: *tb_cell);
    fn tb_change_cell(x: c_int, y: c_int, fg: u16, bg: u16);

    fn tb_blit(x: c_int, y: c_int, w: c_int, h: c_int, cells: *tb_cell);

    fn tb_select_input_mode(mode: input_mode) -> c_int;

    fn tb_peek_event(event: *tb_event, timeout: c_int) -> event_type;
    fn tb_poll_event(event: *tb_event) -> event_type;
  }
}
