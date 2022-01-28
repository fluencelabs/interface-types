#[macro_export]
macro_rules! read_ty {
    ($func_name:ident, $ty:ty, $size:literal) => {
        fn $func_name(&self) -> $ty {
            <$ty>::from_le_bytes(self.read_bytes::<$size>())
        }
    };
}