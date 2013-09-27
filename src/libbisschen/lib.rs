#[crate_type = "lib"];
#[link(name = "bisschen",
       vers = "0.1-pre",
       url = "")];

extern mod extra;

pub mod cbits;
pub mod database;
pub mod threads;
pub mod tags;
pub mod messages;
pub mod query;
