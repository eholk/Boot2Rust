#[no_std];

type EFI_HANDLE = *();

struct EFI_TABLE_HEADER {
    Signature  : u64,
    Revision   : u32,
    HeaderSize : u32,
    CRC32      : u32,
    priv Reserved : u32
}

struct EFI_SYSTEM_TABLE {
    Hdr : EFI_TABLE_HEADER,
}

#[no_mangle]
pub fn efi_main(_ImageHandle : EFI_HANDLE, _SystemTable : EFI_SYSTEM_TABLE) -> int {
    loop {}
}

#[no_mangle]
pub fn __morestack() {
    // Terrible things will happen if this is ever called.
    loop {}
}