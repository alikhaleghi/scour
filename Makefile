.PHONY: all build run docker-build docker-run clean

PORT ?= 10080

all: build

build:
	cargo build --release

run:
	PORT=$(PORT) cargo run --release

docker-build:
	docker build -t scour:latest .

docker-run:
	docker run -d -p $(PORT):$(PORT) -e PORT=$(PORT) --name scour-api scour:latest

clean:
	cargo clean
