use std::cell::Cell;

pub(super) struct ValueReader {
    stream: Vec<u8>,
    offset: Cell<usize>,
}

macro_rules! value_der {
    ($self:expr, $offset:expr, @seq_start $($ids:tt),* @seq_end) => {
        [$($self.stream[$offset + $ids]),+]
    };

    ($self:expr, $offset:expr, 1) => {
        value_der!($self, $offset, @seq_start 0 @seq_end);
    };

    ($self:expr, $offset:expr, 2) => {
        value_der!($self, $offset, @seq_start 0, 1 @seq_end);
    };

    ($self:expr, $offset:expr, 4) => {
        value_der!($self, $offset, @seq_start 0, 1, 2, 3 @seq_end);
    };

    ($self:expr, $offset:expr, 8) => {
        value_der!($self, $offset, @seq_start 0, 1, 2, 3, 4, 5, 6, 7 @seq_end);
    };
}

macro_rules! read_ty {
    ($func_name:ident, $ty:ty, 1) => {
        pub(super) fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(value_der!(self, offset, 1));

            self.offset.set(offset + 1);
            result
        }
    };

    ($func_name:ident, $ty:ty, 2) => {
        pub(super) fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(value_der!(self, offset, 2));

            self.offset.set(offset + 2);
            result
        }
    };

    ($func_name:ident, $ty:ty, 4) => {
        pub(super) fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(value_der!(self, offset, 4));

            self.offset.set(offset + 4);
            result
        }
    };

    ($func_name:ident, $ty:ty, 8) => {
        pub(super) fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(value_der!(self, offset, 8));

            self.offset.set(offset + 8);
            result
        }
    };
}

impl ValueReader {
    pub(super) fn new(stream: Vec<u8>) -> Self {
        let offset = Cell::new(0);
        Self { stream, offset }
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
