#[crate_type = "lib"];
#[link(name = "bisschen",
       vers = "0.1-pre",
       url = "")];

extern mod extra;

pub mod c;
pub mod input;
pub mod termbox;
pub mod database;
pub mod interface;
pub mod tags;
