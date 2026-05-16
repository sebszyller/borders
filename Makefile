.PHONY: install
install:
	cargo build --release
	ln -sf $(PWD)/target/release/borders $(HOME)/.local/bin/borders

.PHONY: build
build:
	cargo build

.PHONY: clean
clean:
	cargo clean
