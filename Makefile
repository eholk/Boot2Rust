.phony: all submodules

all: submodules
	mkdir -p build
	rustc -O -c --lib src/boot.rs -o build/boot.o
	mkdir -p img/efi/boot
	x86_64-efi-pe-ld --oformat pei-x86-64 --subsystem 10 -pie -e efi_start build/boot.o -o img/efi/boot/bootx64.efi
	mkisofs -o boot.iso img

submodules:
	git submodule update --init --recursive
