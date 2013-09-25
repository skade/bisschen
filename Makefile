RUST ?= rust
RUSTC ?= rustc
RUSTFLAGS ?= -O -Z debug-info -L /opt/local/lib -L build --link-args="-lnotmuch -lncurses"
VERSION=0.1-pre

lib_files=\
		      src/c.rs \
		      src/curses.rs \
		      src/input.rs \
		      src/database.rs \
		      src/interface.rs

all: bisschen-tags bisschen-threads

bisschen-tags: lib
	$(RUSTC) $(RUSTFLAGS) src/bisschen-tags.rs --out-dir=build
	
bisschen-threads: lib
	$(RUSTC) $(RUSTFLAGS) src/bisschen-threads.rs --out-dir=build

lib: $(lib_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/lib.rs --out-dir=build