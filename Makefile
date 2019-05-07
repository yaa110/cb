CLI_FILES:=$(shell find cli/src -name '*.rs')
SERVER_FILES:=$(shell find server/src -name '*.rs')
COMMON_FILES:=$(shell find common/src -name '*.rs')

cb: $(SERVER_FILES) $(CLI_FILES) $(COMMON_FILES) Cargo.toml common/Cargo.toml server/Cargo.toml cli/Cargo.toml
	cargo build --release -j`nproc`
	mv -f target/release/cli ./$@
	strip -s $@

clean:
	cargo clean
	rm -f cb
	rm -rf target

uninstall:
	killall cb || exit 0
	rm -f /usr/bin/cb

install: uninstall
	chmod +x cb
	mv -f cb /usr/bin

.PHONY: clean install uninstall
