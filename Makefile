# Zeon Kernel Makefile
# Pure Rust OS kernel for ARM64

.PHONY: all build test run clean image toolchain help

RUST_TARGET := aarch64-unknown-none-softfloat
CARGO_BUILD := cargo build --release --target $(RUST_TARGET)

all: build

build:
	$(CARGO_BUILD)

test:
	cargo test --package libkernel

test-user:
	cargo run --release -- /bin/usertest

run: image
	cargo run --release -- /bin/ash

image:
	./scripts/create-image.sh

clean:
	cargo clean
	rm -f Zeon.img

toolchain:
	./scripts/download-arm-toolchain.sh

help:
	@echo "Zeon Kernel Build System"
	@echo ""
	@echo "Targets:"
	@echo "  make build      - Build the kernel"
	@echo "  make test       - Run unit tests"
	@echo "  make test-user  - Run userspace tests"
	@echo "  make run        - Build and run in QEMU"
	@echo "  make image      - Create disk image"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make toolchain  - Install ARM64 toolchain"
