/*
 * Copyright 2021 Fluence Labs Limited
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

use super::LiError;
use super::LiResult;
use crate::read_array_ty;
use crate::read_ty;
use crate::IValue;

use std::cell::Cell;
use it_utils::MemSlice2;

pub struct MemoryReader<'m> {
    pub(self) memory: MemSlice2<'m>,
}

/// Reads values of basic types sequentially from the provided reader.
/// It doesn't check memory limits for the optimization purposes,
/// so it could be created only by the MemoryReader::sequential_reader method.
pub struct SequentialReader<'r, 'm> {
    reader: &'r MemoryReader<'m>,
    offset: Cell<usize>,
}

impl<'m> MemoryReader<'m> {
    pub fn new(memory: MemSlice2<'m>) -> Self {
        Self { memory }
    }

    /// Returns reader that allows read sequentially. It's important that memory limit is checked
    /// only inside this function. All others functions of the returned reader don't have any
    /// checks assuming that reader is well-formed.
    pub fn sequential_reader(
        &self,
        offset: usize,
        size: usize,
    ) -> LiResult<SequentialReader<'_, '_>> {
        self.check_access(offset, size)?;

        Ok(SequentialReader::new(&self, offset))
    }

    pub fn read_raw_u8_array(&self, offset: usize, elements_count: usize) -> LiResult<Vec<u8>> {
        let reader = self.sequential_reader(offset, elements_count)?;
        let mut result = Vec::with_capacity(elements_count);

        for _ in 0..elements_count {
            let value = reader.read_u8();
            result.push(value);
        }

        Ok(result)
    }

    pub fn read_bool_array(&self, offset: usize, elements_count: usize) -> LiResult<Vec<IValue>> {
        let reader = self.sequential_reader(offset, elements_count)?;
        let mut result = Vec::with_capacity(elements_count);

        for _ in 0..elements_count {
            let value = reader.read_u8();
            result.push(IValue::Boolean(value != 0));
        }

        Ok(result)
    }

    pub fn check_access(&self, offset: usize, size: usize) -> LiResult<()> {
        let right = offset + size;

        // the first condition is a check for overflow
        if right < offset || right >= self.memory.len() {
            return Err(LiError::InvalidAccess {
                offset,
                size,
                memory_size: self.memory.len(),
            });
        }

        Ok(())
    }

    read_array_ty!(read_u8_array, u8, U8);
    read_array_ty!(read_s8_array, i8, S8);
    read_array_ty!(read_u16_array, u16, U16);
    read_array_ty!(read_s16_array, i16, S16);
    read_array_ty!(read_u32_array, u32, U32);
    read_array_ty!(read_s32_array, i32, S32);
    read_array_ty!(read_i32_array, i32, I32);
    read_array_ty!(read_f32_array, f32, F32);
    read_array_ty!(read_u64_array, u64, U64);
    read_array_ty!(read_s64_array, i64, S64);
    read_array_ty!(read_i64_array, i64, I64);
    read_array_ty!(read_f64_array, f64, F64);
}

impl<'r, 'm> SequentialReader<'r, 'm> {
    pub(self) fn new(reader: &'r MemoryReader<'m>, offset: usize) -> Self {
        let offset = Cell::new(offset);
        Self { reader, offset }
    }

    pub fn read_bool(&self) -> bool {
        let offset = self.offset.get();
        let result = self.reader.memory.get(offset) != 0;

        self.offset.set(offset + 1);
        result
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
