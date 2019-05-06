CLI_FILES:=$(shell find cli/src -name '*.rs')
SERVER_FILES:=$(shell find server/src -name '*.rs')
COMMON_FILES:=$(shell find common/src -name '*.rs')

all: server cli

server: cbs

cli: cb

cbs: $(SERVER_FILES) $(COMMON_FILES) Cargo.toml common/Cargo.toml server/Cargo.toml
	cargo build --release -j`nproc` --bin $@
	mv -f target/release/$@ .
	strip -s $@

cb: $(CLI_FILES) $(COMMON_FILES) Cargo.toml common/Cargo.toml cli/Cargo.toml
	cargo build --release -j`nproc` --bin $@
	mv -f target/release/$@ .
	strip -s $@

clean:
	cargo clean
	rm -f cb cbs
	rm -rf target

uninstall:
	killall cbs || exit 0
	rm -f /usr/bin/cbs /usr/bin/cb

install: uninstall
	chmod +x cb
	chmod +x cbs
	mv -f cbs cb /usr/bin

.PHONY: clean install uninstall
