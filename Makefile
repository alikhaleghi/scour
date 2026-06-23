.PHONY: all build run clean
PORT ?= 10080
all: build
build: cargo build --release
run: PORT=$(PORT) cargo run --release
clean: cargo clean
