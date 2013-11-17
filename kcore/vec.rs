use kcore::traits::Container;

impl<'self, T> Container for &'self [T] {
    fn len(&self) -> uint {
        let (_, l) : (*T, uint) = unsafe { transmute(*self) };
        l
    }
}

extern "rust-intrinsic" {
    fn transmute<T,U>(val: T) -> U;
}
