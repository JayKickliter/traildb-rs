.PHONY: build clean test

all: build

build:
	cargo build

test:
	cargo test -- --nocapture

clean:
	cargo clean
