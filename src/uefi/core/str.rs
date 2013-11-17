use uefi::core::traits::Container;

impl<'self> Container for &'self str {
    #[no_split_stack]
    fn len(&self) -> uint {
        let (_, l) : (*u8, uint) = unsafe { transmute(*self) };
        l
    }
}

extern "rust-intrinsic" {
    fn transmute<T,U>(val: T) -> U;
}
