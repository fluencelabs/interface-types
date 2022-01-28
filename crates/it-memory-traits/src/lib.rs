mod errors;

pub use errors::MemoryAccessError;

macro_rules! read_ty {
    ($func_name:ident, $ty:ty, $size:literal) => {
        fn $func_name(&self) -> $ty {
            <$ty>::from_le_bytes(self.read_bytes::<$size>())
        }
    };
}

pub trait SequentialReader {
    fn read_byte(&self) -> u8;

    fn read_bytes<const COUNT: usize>(&self) -> [u8; COUNT];

    fn read_bool(&self) -> bool {
        self.read_byte() != 0
    }

    read_ty!(read_u8, u8, 1);
    read_ty!(read_i8, i8, 1);
    read_ty!(read_u16, u16, 2);
    read_ty!(read_i16, i16, 2);
    read_ty!(read_u32, u32, 4);
    read_ty!(read_i32, i32, 4);
    read_ty!(read_f32, f32, 4);
    read_ty!(read_u64, u64, 8);
    read_ty!(read_i64, i64, 8);
    read_ty!(read_f64, f64, 8);
}

pub trait SequentialWriter {
    fn start_offset(&self) -> usize;

    // specialization of write_array for u8
    fn write_u8(&self, value: u8);

    // specialization of write_array for u32
    fn write_u32(&self, value: u32);

    fn write_bytes(&self, bytes: &[u8]);
}

pub trait SequentialMemoryView<'a> {
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
    View: for<'a> SequentialMemoryView<'a>,
{
    fn view(&self) -> View;
}
