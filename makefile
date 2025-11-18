default: run
all: build run

build: format
	cargo build --release

run: format
	cargo run --release

test: format
	cargo test

format:
	cargo fmt

check: format
	cargo check