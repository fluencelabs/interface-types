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

use super::LiResult;
use crate::read_array_ty;
use crate::IValue;

use it_traits::{MemoryView, SequentialReader};

pub struct MemoryReader<MV: MemoryView> {
    pub(self) view: MV,
}

impl<MV: MemoryView> MemoryReader<MV> {
    pub fn new(view: MV) -> Self {
        Self { view }
    }

    /// Returns reader that allows read sequentially. It's important that memory limit is checked
    /// only inside this function. All others functions of the returned reader don't have any
    /// checks assuming that reader is well-formed.
    pub fn sequential_reader<'s>(
        &'s self,
        offset: usize,
        size: usize,
    ) -> LiResult<Box<dyn SequentialReader + 's>> {
        let seq_reader = self.view.sequential_reader(offset, size);
        Ok(seq_reader)
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
