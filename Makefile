.PHONY: build clean test wiki

all: build

build:
	cargo build

test: assets/wikipedia-history-small.tdb
	cargo test -- --nocapture

clean:
	cargo clean

assets/wikipedia-history-small.tdb:
	mkdir -p assets
	wget http://traildb.io/data/wikipedia-history-small.tdb -O assets/wikipedia-history-small.tdb

example: assets/wikipedia-history-small.tdb
	cargo run --example wiki
