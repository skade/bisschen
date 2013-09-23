test: database-test
	./database

bisschen-threads:
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch -lncurses" bisschen-threads.rs

interface: c database
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch -lncurses" interface.rs

database: c
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch" database.rs

database-test: c
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch" --test database.rs

c:
	rustc -L. -L /opt/local/lib --link-args="-lnotmuch" c.rs