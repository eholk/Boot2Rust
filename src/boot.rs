#[no_std];

use uefi::SimpleTextOutput;

pub mod core;
pub mod uefi;

#[no_split_stack]
pub fn efi_main(sys : uefi::SystemTable) {
    sys.console().write("Hello, World!\n\r");

    loop {}
}
