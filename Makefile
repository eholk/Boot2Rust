.phony: all
all : 
	rustc -O -c --lib boot.rs
	x86_64-efi-pe-ld -g --oformat pei-x86-64 --dll --subsystem 10:1.0 --major-os-version 0 --heap 0,0 -nostdlib --stack 0x1000,0 -pie -Bsymbolic -nocombreloc -e efi_main boot.o -o boot.efi

