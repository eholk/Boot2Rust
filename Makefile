.phony: all
all : 
	mkdir -p build
	rustc -O -c --lib src/boot.rs -o build/boot.o
	mkdir -p img/efi/boot
	x86_64-efi-pe-ld -g --oformat pei-x86-64 --dll --subsystem 10:1.0 --major-os-version 0 --heap 0,0 -nostdlib --stack 0x200000,0 -pie -Bsymbolic -nocombreloc -e efi_start build/boot.o -o img/efi/boot/bootx64.efi
	mkisofs -o boot.iso img
