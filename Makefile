LD := x86_64-efi-pe-ld

.phony: all submodules image rust-core

all: submodules image

image: rust-core
	mkdir -p build
	rustc -O --emit=obj --crate-type=lib src/boot.rs --out-dir build/ -L build/
	mkdir -p img/efi/boot
	$(LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_start build/boot.o build/core.o -o img/efi/boot/bootx64.efi
	mkisofs -o boot.iso img

submodules:
	# git submodule update --init --recursive

rust-core:
	rustc --crate-type=rlib --emit=obj,link --out-dir build/ external/rust-core/core/lib.rs
