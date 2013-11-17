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
    FirmwareVendor : *u16,
    FirmwareRevision : u32,
    ConsoleInHandle : EFI_HANDLE,
    ConIn : *EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle : EFI_HANDLE,
    ConOut : *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    // ... other stuff that we're ignoring for now.
}

struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset : EFI_TEXT_RESET,
    OutputString : EFI_TEXT_STRING,
    // ... and more stuff that we're ignoring.
}

type EFI_TEXT_RESET = *();

type EFI_TEXT_STRING = extern "win64" fn(*EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
                                            *u16);

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_main(_ImageHandle : EFI_HANDLE, SystemTable : *EFI_SYSTEM_TABLE) -> int {
    unsafe {
        let SystemTable = *SystemTable;
        let vendor = SystemTable.FirmwareVendor;
        let conout = SystemTable.ConOut;
        let output = (*conout).OutputString;

        let hello = ['H' as u16,
                     'e' as u16,
                     'l' as u16,
                     'l' as u16,
                     'o' as u16,
                     ',' as u16,
                     ' ' as u16,
                     'W' as u16,
                     'o' as u16,
                     'r' as u16,
                     'l' as u16,
                     'd' as u16,
                     '\r' as u16,
                     '\n' as u16,
                     0u16];
        let (hello_ptr, _) = buf_ptr(hello);

        output(conout, hello_ptr);

        loop {
        }
    }
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
//    pub fn size_of<T>() -> uint;
}
