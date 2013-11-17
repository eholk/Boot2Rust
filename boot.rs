#[no_std];

use uefi::SimpleTextOutput;

mod uefi;

#[no_split_stack]
pub fn efi_main(sys : uefi::SystemTable) {
    let hello = "H\0e\0l\0l\0o\0,\0 \0W\0o\0r\0l\0d\0!\0\n\0\r\0";

    unsafe {
        sys.console().write(str_as_u16_slice(hello));

        loop {
        }
    }
}

unsafe fn str_as_u16_slice(s: &str) -> &[u16] {
        let (p, l) : (*u8, uint) = transmute(s);
        transmute((p, l >> 1))
}

fn buf_ptr<T>(buf: &[T]) -> (*T, uint) {
    unsafe { transmute(buf) }
}

//#[no_mangle]
//#[no_split_stack]
//pub fn __morestack() {
//    // If this is called, something horrible probably happened...
//}

extern "rust-intrinsic" {
    pub fn transmute<T,U>(val: T) -> U;
}

// Ideally this next function would be in uefi.rs, but the linker
// forgets about it there.
#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_start(_ImageHandle : uefi::EFI_HANDLE,
                                sys_table : *uefi::EFI_SYSTEM_TABLE) -> int {
    efi_main(uefi::SystemTable(sys_table));
    0
}
