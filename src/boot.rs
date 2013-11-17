#[no_std];

use uefi::SimpleTextOutput;

mod uefi;

#[no_split_stack]
pub fn efi_main(sys : uefi::SystemTable) {
    sys.console().write("Hello, World!\n\r");

    loop {}
}

// Ideally this next function would be in uefi.rs, but the linker
// forgets about it there.
#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_start(_ImageHandle : uefi::EFI_HANDLE,
                                sys_table : *uefi::EFI_SYSTEM_TABLE) -> int {
    unsafe { uefi::SYSTEM_TABLE = sys_table; }
    efi_main(uefi::SystemTable(sys_table));
    0
}
