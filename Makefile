.POSIX:
.PHONY: default dev fmt test test-unit lint update

default: fmt lint target/release/bunku

target/release/bunku: src/ Cargo.toml
	cargo build --release

dev:
	cargo run

fmt:
	cargo fmt

test: lint test-unit

test-unit:
	cargo test

lint:
	cargo clippy -- --deny warnings

update:
	nix flake update
	cargo update
