default: run
all: build run

build: format
	cargo build --release

run: build
	./target/release/sim
	

test: format
	cargo test

format:
	cargo fmt

check: format
	cargo check