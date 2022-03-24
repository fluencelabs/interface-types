/*
 * Copyright 2022 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
    fn start_offset(&self) -> u32;

    // specialization of write_array for u8
    fn write_u8(&self, value: u8);

    // specialization of write_array for u32
    fn write_u32(&self, value: u32);

    fn write_bytes(&self, bytes: &[u8]);
}

// the lifetime is needed because some implementations
// need to bind SR and SW lifetimes to lifetime of &self in methods
pub trait SequentialMemoryView<'s> {
    type SR: SequentialReader + 's;
    type SW: SequentialWriter + 's;

    fn sequential_writer(&'s self, offset: u32, size: u32) -> Result<Self::SW, MemoryAccessError>;

    fn sequential_reader(&'s self, offset: u32, size: u32) -> Result<Self::SR, MemoryAccessError>;
}

pub trait Memory<View>
where
    View: for<'a> SequentialMemoryView<'a>,
{
    fn view(&self) -> View;
}
