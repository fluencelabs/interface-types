use thiserror::Error as ThisError;

#[macro_export]
macro_rules! read_ty_decl {
    ($func_name:ident, $ty:ty, 1) => {
        fn $func_name(&self) -> $ty;
    };

    ($func_name:ident, $ty:ty, 2) => {
        fn $func_name(&self) -> $ty;
    };

    ($func_name:ident, $ty:ty, 4) => {
        fn $func_name(&self) -> $ty;
    };

    ($func_name:ident, $ty:ty, 8) => {
        fn $func_name(&self) -> $ty;
    };

    ($func_name:ident, $ty:ty, 16) => {
        fn $func_name(&self) -> $ty;
    };
}

pub trait SequentialReader {
    fn read_bool(&self) -> bool;

    read_ty_decl!(read_u8, u8, 1);
    read_ty_decl!(read_i8, i8, 1);
    read_ty_decl!(read_u16, u16, 2);
    read_ty_decl!(read_i16, i16, 2);
    read_ty_decl!(read_u32, u32, 4);
    read_ty_decl!(read_i32, i32, 4);
    read_ty_decl!(read_f32, f32, 4);
    read_ty_decl!(read_u64, u64, 8);
    read_ty_decl!(read_i64, i64, 8);
    read_ty_decl!(read_f64, f64, 8);
}

pub trait SequentialWriter {
    fn start_offset(&self) -> usize;

    // specialization of write_array for u8
    fn write_u8(&self, value: u8);

    // specialization of write_array for u32
    fn write_u32(&self, value: u32);

    fn write_bytes(&self, bytes: &[u8]);
}

pub trait MemoryView<'a> {
    type SR: SequentialReader + 'a;
    type SW: SequentialWriter + 'a;

    fn sequential_writer(
        &'a self,
        offset: usize,
        size: usize,
    ) -> Result<Self::SW, MemoryAccessError>;

    fn sequential_reader(
        &'a self,
        offset: usize,
        size: usize,
    ) -> Result<Self::SR, MemoryAccessError>;
}

pub trait Memory<View>
where
    View: for<'a> MemoryView<'a>,
{
    fn view(&self) -> View;
}

#[derive(Debug, ThisError)]
pub enum MemoryAccessError {
    #[error("Out-of-bound Wasm memory access: offset {offset}, size {size}, while memory_size {memory_size}")]
    OutOfBounds {
        offset: usize,
        size: usize,
        memory_size: usize,
    },
}
