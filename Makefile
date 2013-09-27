RUST ?= rust
RUSTC ?= rustc
RUSTFLAGS ?= -O -Z debug-info -L build -L build/notmuch/lib -L build/termbox/lib --link-args="-lnotmuch -ltermbox"
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

demo: lib
	$(RUSTC) $(RUSTFLAGS) src/demo.rs --out-dir=build

bisschen-tags: lib
	$(RUSTC) $(RUSTFLAGS) src/bisschen-tags.rs --out-dir=build
	
bisschen-threads: lib
	$(RUSTC) $(RUSTFLAGS) src/bisschen-threads.rs --out-dir=build

lib: $(notmuch_files) $(termbox_files) $(lib_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/lib.rs --out-dir=build

$(termbox_files):
	mkdir -p build/termbox
	cd termbox && ./waf configure --prefix=/ && ./waf && ./waf install --targets=termbox_static --destdir=../build/termbox

$(notmuch_files):
	mkdir -p build/notmuch
	cd notmuch && ./configure --prefix=$(CURDIR)/build/notmuch --without-emacs --without-bash-completion --without-zsh-completion && make && make install

iterator-test:
	$(RUSTC) $(RUSTFLAGS) src/caching_iterator.rs --test --out-dir=build
	build/caching_iterator

tags-test:
	$(RUSTC) $(RUSTFLAGS) src/tags.rs --test --out-dir=build
	build/tags

threads-test:
	$(RUSTC) $(RUSTFLAGS) src/threads.rs --test --out-dir=build
	build/threads

clean:
	git clean -f -d -X
