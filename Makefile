.PHONY: link
link:
	cargo build --release
	ln -sf $(PWD)/target/release/borders ~/.local/bin/borders

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	rm -f *.jpg
	rm -rf resized
	cargo clean
