.phony: all submodules image rust-core

all: submodules image

image: rust-core
	mkdir -p build
	rustc -O -c --lib src/boot.rs -L external/rust-core/core
	mkdir -p img/efi/boot
	x86_64-efi-pe-ld --oformat pei-x86-64 --subsystem 10 -pie -e efi_start src/boot.o  external/rust-core/core/core.o -o img/efi/boot/bootx64.efi
	mkisofs -o boot.iso img

submodules:
	git submodule update --init --recursive

rust-core:
	rustc -c --lib external/rust-core/core/lib.rs