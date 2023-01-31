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
use crate::read_ty;
use crate::IValue;

use it_memory_traits::MemoryView;

use std::cell::Cell;
use std::marker::PhantomData;

pub struct MemoryReader<MV: MemoryView<Store>, Store: it_memory_traits::Store> {
    pub(self) view: MV,
    _phantom: PhantomData<Store>,
}

impl<MV: MemoryView<Store>, Store: it_memory_traits::Store> MemoryReader<MV, Store> {
    pub fn new(view: MV) -> Self {
        Self {
            view,
            _phantom: PhantomData,
        }
    }

    /// Returns reader that allows read sequentially. It's important that memory limit is checked
    /// only inside this function. All others functions of the returned reader don't have any
    /// checks assuming that reader is well-formed.
    pub fn sequential_reader(
        &self,
        store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
        offset: u32,
        size: u32,
    ) -> LiResult<SequentialReader<'_, MV, Store>> {
        self.view.check_bounds(store, offset, size)?;
        let seq_reader = SequentialReader::new(&self, offset);
        Ok(seq_reader)
    }

    pub fn read_raw_u8_array(
        &self,
        store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
        offset: u32,
        elements_count: u32,
    ) -> LiResult<Vec<u8>> {
        let reader = self.sequential_reader(store, offset, elements_count)?;
        let mut result = Vec::with_capacity(elements_count as usize);

        for _ in 0..elements_count {
            let value = reader.read_u8(store);
            result.push(value);
        }

        Ok(result)
    }

    pub fn read_bool_array(
        &self,
        store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
        offset: u32,
        elements_count: u32,
    ) -> LiResult<Vec<IValue>> {
        let reader = self.sequential_reader(store, offset, elements_count)?;
        let mut result = Vec::with_capacity(elements_count as usize);

        for _ in 0..elements_count {
            let value = reader.read_u8(store);
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

pub struct SequentialReader<'r, MV: MemoryView<Store>, Store: it_memory_traits::Store> {
    reader: &'r MemoryReader<MV, Store>,
    offset: Cell<u32>,
}

impl<'r, MV: MemoryView<Store>, Store: it_memory_traits::Store> SequentialReader<'r, MV, Store> {
    fn new(reader: &'r MemoryReader<MV, Store>, offset: u32) -> Self {
        Self {
            reader,
            offset: Cell::new(offset),
        }
    }

    pub fn read_bool(
        &self,
        store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    ) -> bool {
        self.read_u8(store) != 0
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
