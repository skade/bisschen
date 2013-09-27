RUST ?= rust
RUSTC ?= rust build
RUSTTEST ?= rust test
RUSTFLAGS ?= -O -Z debug-info --out-dir build -L build -L build/notmuch/lib -L build/termbox/lib --link-args="-lnotmuch -ltermbox"
VERSION=0.1-pre

lib_files=\
		      src/c.rs \
		      src/input.rs \
		      src/database.rs \
		      src/interface.rs \
		      src/caching_iterator.rs \
		      src/termbox.rs

termbox_files=\
					build/termbox/lib/libtermbox.a

notmuch_files=\
					build/notmuch/lib/libnotmuch.dylib

all: bisschen-tags bisschen-threads

bisschen-tags: lib
	$(RUSTC) $(RUSTFLAGS) src/bisschen-tags.rs

bisschen-threads: lib
	$(RUSTC) $(RUSTFLAGS) src/bisschen-threads.rs

lib: $(notmuch_files) $(termbox_files) $(lib_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/lib.rs

$(termbox_files):
	mkdir -p build/termbox
	cd termbox && ./waf configure --prefix=/ && ./waf && ./waf install --targets=termbox_static --destdir=../build/termbox

$(notmuch_files):
	mkdir -p build/notmuch
	cd notmuch && ./configure --prefix=$(CURDIR)/build/notmuch --without-emacs --without-bash-completion --without-zsh-completion && make && make install

test:
	$(RUSTC) $(RUSTFLAGS) --test src/lib.rs
	src/lib

clean:
	git clean -f -d -X
