#[no_std];

#[feature(asm)];
#[feature(macro_rules)];

extern mod core;

use uefi::SimpleTextOutput;

pub mod uefi;

#[no_split_stack]
pub fn efi_main(sys : uefi::SystemTable) {
    sys.console().write("Hello, World!\n\r");

    loop {}
}
