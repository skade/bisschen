pub mod termbox {
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

  pub enum event_type {
    TB_EVENT_KEY = 1,
    TB_EVENT_RESIZE = 2,
  }

  enum input_mode {
    TB_INPUT_CURRENT = 0,
    TB_INPUT_ESC = 1,
    TB_INPUT_ALT = 2,
  }

  pub enum keys {
    TB_KEY_CTRL_I = 0x09,
    TB_KEY_CTRL_J = 0x0A,
    TB_KEY_CTRL_K = 0x0B,
    TB_KEY_CTRL_L = 0x0C,
    TB_KEY_ENTER  = 0x0D,
  }

  extern {
    pub fn tb_init() -> c_int;
    pub fn tb_shutdown();

    pub fn tb_width() -> c_int;
    pub fn tb_height() -> c_int;

    pub fn tb_clear();
    pub fn tb_set_clear_attributes(fg: u16, bg: u16);

    pub fn tb_present();

    pub fn tb_set_cursor(cx: c_int, cy: c_int);

    pub fn tb_put_cell(x: c_int, y: c_int, cell: *tb_cell);
    pub fn tb_change_cell(x: c_int, y: c_int, fg: u16, bg: u16);

    pub fn tb_blit(x: c_int, y: c_int, w: c_int, h: c_int, cells: *tb_cell);

    pub fn tb_select_input_mode(mode: input_mode) -> c_int;

    pub fn tb_peek_event(event: *tb_event, timeout: c_int) -> event_type;
    pub fn tb_poll_event(event: *tb_event) -> event_type;
  }
}
