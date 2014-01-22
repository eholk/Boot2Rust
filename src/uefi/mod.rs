use core::container::Container;

pub type EFI_HANDLE = *();
pub struct EFI_GUID(u32, u16, u16, [u8, ..8]);

struct EFI_TABLE_HEADER {
    Signature  : u64,
    Revision   : u32,
    HeaderSize : u32,
    CRC32      : u32,
    priv Reserved : u32
}

pub struct EFI_SYSTEM_TABLE {
    Hdr : EFI_TABLE_HEADER,
    FirmwareVendor : *u16,
    FirmwareRevision : u32,
    ConsoleInHandle : EFI_HANDLE,
    ConIn : *EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle : EFI_HANDLE,
    ConOut : *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ConsoleErrorHandle : EFI_HANDLE,
    StdErr : *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    RuntimeServices : *EFI_RUNTIME_SERVICES,
    BootServices : *EFI_BOOT_SERVICES,
    NumberOfTableEntries : uint,
    ConfigurationTable : *EFI_CONFIGURATION_TABLE
}

pub static mut SYSTEM_TABLE : *EFI_SYSTEM_TABLE = 0 as *EFI_SYSTEM_TABLE;

struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset : EFI_TEXT_RESET,
    OutputString : EFI_TEXT_STRING,
    // ... and more stuff that we're ignoring.
}

type EFI_TEXT_RESET = *();

type EFI_TEXT_STRING = extern "win64" fn(*EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
                                         *u16);

struct EFI_RUNTIME_SERVICES;

struct EFI_BOOT_SERVICES;

struct EFI_CONFIGURATION_TABLE {
    VendorGuid : EFI_GUID,
    VendorTable : *()
}

pub struct SystemTable(*EFI_SYSTEM_TABLE);


impl SystemTable {
    #[no_split_stack]
    pub fn console(&self) -> Console {
        unsafe {
            let &SystemTable(tbl) = self;
            Console {
                input:  (*tbl).ConIn,
                output: (*tbl).ConOut,
            }
        }
    }
}

fn unpack<T>(slice: &[T]) -> (*T, uint) {
    unsafe {
        transmute(slice)
    }
}

pub trait SimpleTextOutput {
    unsafe fn write_raw(&self, str: *u16);
    
    #[no_split_stack]
    fn write(&self, str: &str) {
        let mut buf = [0u16, ..4096];

        let mut i = 0;
        while i < buf.len() && i < str.len() {
            // TODO: make sure the characters are all ascii
            buf[i] = str[i] as u16;
            i += 1;
        }
        buf[buf.len() - 1] = 0;
        
        unsafe {
            let (p, _) = unpack(buf);
            self.write_raw(p);
        }
    }
}

pub trait SimpleTextInput {
}

pub struct Console {
    priv input  : *EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    priv output : *EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
}

impl SimpleTextOutput for Console {
    #[no_split_stack]
    unsafe fn write_raw(&self, str: *u16) {
        ((*(*self).output).OutputString)(self.output, str);
    }
}

impl SimpleTextInput for Console {
}

extern "rust-intrinsic" {
    fn transmute<T,U>(val: T) -> U;
}

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_start(_ImageHandle : EFI_HANDLE,
                                sys_table : *EFI_SYSTEM_TABLE) -> int {
    unsafe { SYSTEM_TABLE = sys_table; }
    ::efi_main(SystemTable(sys_table));
    0
}

#[no_mangle]
#[no_split_stack]
pub fn __morestack() {
    // Horrible things will probably happen if this is ever called.
}

#[no_mangle]
#[no_split_stack]
pub extern fn memset(s : *u8, c : int, n : uint) -> *u8 {
    unsafe {
        let s : &mut [u8] = transmute((s, n));
        let mut i = 0;
        while i < n {
            s[i] = c as u8;
            i += 1;
            // Use inline assembly here to defeat LLVM's loop-idiom pass
            asm!("");
        }
    }

    s
}
