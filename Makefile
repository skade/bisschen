RUST ?= rust
RUSTC ?= rustc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build -L build/notmuch/lib -L build/termbox/lib -C link-args="-ltermbox -lnotmuch"
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

all: bisschen-tags bisschen-threads bisschen-thread

bisschen: libbisschen
	$(RUSTC) $(RUSTFLAGS) src/bisschen/bisschen.rs

bisschen-tags: libbisschen libtermbox
	$(RUSTC) $(RUSTFLAGS) src/bisschen/bisschen-tags.rs

bisschen-threads: libbisschen libtermbox
	$(RUSTC) $(RUSTFLAGS) src/bisschen/bisschen-threads.rs

bisschen-thread: libbisschen libtermbox
	$(RUSTC) $(RUSTFLAGS) src/bisschen/bisschen-thread.rs

libbisschen: $(notmuch_files) $(termbox_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/libbisschen/lib.rs

libtermbox: $(notmuch_files) $(termbox_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/libtermbox/lib.rs

$(termbox_files):
	mkdir -p build/termbox
	cd termbox && ./waf configure --prefix=/ && ./waf && ./waf install --targets=termbox_static --destdir=../build/termbox

$(notmuch_files):
	mkdir -p build/notmuch
	cd notmuch && ./configure --prefix=$(CURDIR)/build/notmuch --without-emacs --without-bash-completion --without-zsh-completion && make && make install

test: libbisschen-test

libbisschen-test: $(notmuch_files) $(termbox_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) --test src/libbisschen/lib.rs
	build/bisschen

clean:
	git clean -f -d -X
