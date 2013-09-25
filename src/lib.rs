#[crate_type = "lib"];
#[link(name = "bisschen",
       vers = "0.1-pre",
       uuid = "d2ad8df0-547a-4ce1-99c6-a9da3b98fb3e",
       url = "")];

extern mod extra;

pub mod c;
pub mod input;
pub mod curses;
pub mod termbox;
pub mod database;
pub mod interface;
