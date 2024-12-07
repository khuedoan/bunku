.POSIX:
.PHONY: default dev fmt test test-unit lint update docs-serve ci

default: fmt lint target/release/bunku

target/release/bunku: src/ Cargo.toml
	cargo build --release

dev:
	cargo run -- --filename ./examples/podinfo/app.toml

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

docs-serve:
	cd docs && mdbook serve

docs/book: docs
	cd docs && mdbook build

ci: docs/book
