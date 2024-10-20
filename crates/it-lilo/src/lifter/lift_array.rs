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

use super::record_lift_memory;
use super::ILifter;
use super::LiResult;
use crate::traits::RecordResolvable;
use crate::utils::ser_type_size;
use crate::IType;
use crate::IValue;

use it_memory_traits::MemoryView;

pub fn array_lift_memory<
    R: RecordResolvable,
    MV: MemoryView<Store>,
    Store: it_memory_traits::Store,
>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lifter: &ILifter<'_, R, MV, Store>,
    value_type: &IType,
    offset: u32,
    elements_count: u32,
) -> LiResult<IValue> {
    if elements_count == 0 {
        return Ok(IValue::Array(vec![]));
    }

    let reader = &lifter.reader;

    let ivalues = match value_type {
        IType::Boolean => reader.read_bool_array(store, offset, elements_count)?,
        IType::S8 => reader.read_s8_array(store, offset, elements_count)?,
        IType::S16 => reader.read_s16_array(store, offset, elements_count)?,
        IType::S32 => reader.read_s32_array(store, offset, elements_count)?,
        IType::S64 => reader.read_s64_array(store, offset, elements_count)?,
        IType::I32 => reader.read_i32_array(store, offset, elements_count)?,
        IType::I64 => reader.read_i64_array(store, offset, elements_count)?,
        IType::U8 => reader.read_u8_array(store, offset, elements_count)?,
        IType::U16 => reader.read_u16_array(store, offset, elements_count)?,
        IType::U32 => reader.read_u32_array(store, offset, elements_count)?,
        IType::U64 => reader.read_u64_array(store, offset, elements_count)?,
        IType::F32 => reader.read_f32_array(store, offset, elements_count)?,
        IType::F64 => reader.read_f64_array(store, offset, elements_count)?,
        IType::String => read_string_array(store, lifter, offset, elements_count)?,
        IType::ByteArray => read_array_array(store, lifter, &IType::U8, offset, elements_count)?,
        IType::Array(ty) => read_array_array(store, lifter, &ty, offset, elements_count)?,
        IType::Record(record_type_id) => {
            read_record_array(store, lifter, *record_type_id, offset, elements_count)?
        }
    };

    Ok(IValue::Array(ivalues))
}

fn read_string_array<R: RecordResolvable, MV: MemoryView<Store>, Store: it_memory_traits::Store>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lifter: &ILifter<'_, R, MV, Store>,
    offset: u32,
    elements_count: u32,
) -> LiResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count as usize);
    let seq_reader = lifter.reader.sequential_reader(
        store,
        offset,
        ser_type_size(&IType::String) * elements_count,
    )?;

    for _ in 0..elements_count {
        let offset = seq_reader.read_u32(store);
        let size = seq_reader.read_u32(store);

        let raw_str = lifter.reader.read_raw_u8_array(store, offset, size)?;
        let str = String::from_utf8(raw_str)?;
        result.push(IValue::String(str));
    }

    Ok(result)
}

fn read_array_array<R: RecordResolvable, MV: MemoryView<Store>, Store: it_memory_traits::Store>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lifter: &ILifter<'_, R, MV, Store>,
    ty: &IType,
    offset: u32,
    elements_count: u32,
) -> LiResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count as usize);
    let seq_reader =
        lifter
            .reader
            .sequential_reader(store, offset, ser_type_size(ty) * elements_count)?;

    for _ in 0..elements_count {
        let offset = seq_reader.read_u32(store);
        let size = seq_reader.read_u32(store);

        let array = array_lift_memory(store, lifter, ty, offset, size)?;
        result.push(array);
    }

    Ok(result)
}

fn read_record_array<R: RecordResolvable, MV: MemoryView<Store>, Store: it_memory_traits::Store>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lifter: &ILifter<'_, R, MV, Store>,
    record_type_id: u64,
    offset: u32,
    elements_count: u32,
) -> LiResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count as usize);
    let seq_reader = lifter.reader.sequential_reader(
        store,
        offset,
        ser_type_size(&IType::Record(0)) * elements_count,
    )?;

    for _ in 0..elements_count {
        let offset = seq_reader.read_u32(store);
        let record_ty = lifter.resolver.resolve_record(record_type_id)?;

        let record = record_lift_memory(store, lifter, &record_ty, offset)?;
        result.push(record);
    }

    Ok(result)
}
