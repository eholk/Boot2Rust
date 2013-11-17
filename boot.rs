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

        let hello = "H\0e\0l\0l\0o\0,\0 \0W\0o\0r\0l\0d\0!\0\n\0\r\0";

        output(conout, str_as_u16(hello));

        loop {
        }
    }
}

fn str_as_u16(s: &str) -> *u16 {
    unsafe {
        let (p, _) : (*u8, uint) = transmute(s);
        transmute(p)
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
