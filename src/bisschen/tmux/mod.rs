use std::run::*;

pub fn set(key: ~str, value: ~str) {
  let mut setter = Process::new("tmux", [~"set", ~"@" + key, value], ProcessOptions::new());
  setter.finish();
}