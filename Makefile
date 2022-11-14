.PHONY: run
run:
	cargo run

.PHONY: build
build:
	cargo build --release

.PHONY: check
check:
	cargo check

.PHONY: fmt
fmt:
	cargo fmt
