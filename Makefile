RUST ?= rust
RUSTC ?= rustc
RUSTFLAGS ?= -O -Z debug-info -L /opt/local/lib -L build -L build/termbox/lib --link-args="-lncurses -lnotmuch -ltermbox"
VERSION=0.1-pre

lib_files=\
		      src/c.rs \
		      src/curses.rs \
		      src/input.rs \
		      src/database.rs \
		      src/interface.rs \
          src/termbox.rs

termbox_files=\
					build/termbox/libtermbox.a

all: bisschen-tags bisschen-threads

demo: lib
	$(RUSTC) $(RUSTFLAGS) src/demo.rs --out-dir=build

bisschen-tags: lib
	$(RUSTC) $(RUSTFLAGS) src/bisschen-tags.rs --out-dir=build
	
bisschen-threads: lib
	$(RUSTC) $(RUSTFLAGS) src/bisschen-threads.rs --out-dir=build

lib: $(termbox_files) $(lib_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/lib.rs --out-dir=build

$(termbox_files):
	mkdir -p build/termbox
	cd termbox && ./waf configure --prefix=/ && ./waf && ./waf install --targets=termbox_static --destdir=../build/termbox

clean:
	git clean -f -d -X
