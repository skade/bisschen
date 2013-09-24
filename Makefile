test: database-test
	./database

demo: c database
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch -lncurses" demo.rs

bisschen-threads:
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch -lncurses" bisschen-threads.rs

interface: c database input curses
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch -lncurses" interface.rs

database: c
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch" database.rs

database-test: c
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch" --test database.rs

c:
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch" c.rs

input: c
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch -lncurses" input.rs

curses: c
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch -lncurses" curses.rs