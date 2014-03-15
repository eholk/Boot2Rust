#[no_std];

#[feature(asm)];
#[feature(macro_rules)];

extern crate core;

use uefi::SimpleTextOutput;

pub mod uefi;

#[no_split_stack]
pub fn efi_main(sys : uefi::SystemTable) {
    sys.console().write("Hello, World!\n\r");

    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
	loop {}
}

#[no_mangle]
pub fn breakpoint() -> ! {
	loop {}
}
