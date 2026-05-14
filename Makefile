.PHONY: link
link:
	cargo build --release

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	rm -f *.jpg
	rm -rf resized
	cargo clean
