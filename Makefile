.PHONY: link
link:
	cargo build --release
	ln -sf $(PWD)/target/release/borders $(HOME)/.local/bin/borders

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	cargo clean
