.PHONY: build check test run clean

build:
	cargo build

check:
	cargo check

test:
	cargo test

run:
	cargo run -- $(ARGS)

clean:
	cargo clean
