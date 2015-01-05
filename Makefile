LD := x86_64-efi-pe-ld

.phony: all image

all: image

image:
	mkdir -p build
	rustc -O --emit=obj --crate-type=lib src/boot.rs --out-dir build/
	mkdir -p img/efi/boot
	$(LD) --oformat pei-x86-64 --subsystem 10 -pie -e efi_start build/boot.o -o img/efi/boot/bootx64.efi
	mkisofs -o boot.iso img
