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

use super::ILifter;
use super::LiError;
use super::LiResult;
use super::MemoryReader;
use crate::lifter::memory_reader::SequentialReader;
use crate::traits::RecordResolvable;
use crate::utils::record_size;
use crate::IRecordType;
use crate::IType;
use crate::IValue;
use crate::NEVec;

use it_memory_traits::MemoryView;

pub fn record_lift_memory<
    R: RecordResolvable,
    MV: MemoryView<Store>,
    Store: it_memory_traits::Store,
>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lifter: &ILifter<'_, R, MV, Store>,
    record_type: &IRecordType,
    offset: u32,
) -> LiResult<IValue> {
    let mut values = Vec::with_capacity(record_type.fields.len());

    let size = record_size(record_type);
    let reader = &lifter.reader;
    let seq_reader = reader.sequential_reader(store, offset, size)?;

    for field in (*record_type.fields).iter() {
        match &field.ty {
            IType::Boolean => values.push(IValue::Boolean(seq_reader.read_u8(store) != 0)),
            IType::S8 => values.push(IValue::S8(seq_reader.read_i8(store))),
            IType::S16 => values.push(IValue::S16(seq_reader.read_i16(store))),
            IType::S32 => values.push(IValue::S32(seq_reader.read_i32(store))),
            IType::S64 => values.push(IValue::S64(seq_reader.read_i64(store))),
            IType::I32 => values.push(IValue::I32(seq_reader.read_i32(store))),
            IType::I64 => values.push(IValue::I64(seq_reader.read_i64(store))),
            IType::U8 => values.push(IValue::U8(seq_reader.read_u8(store))),
            IType::U16 => values.push(IValue::U16(seq_reader.read_u16(store))),
            IType::U32 => values.push(IValue::U32(seq_reader.read_u32(store))),
            IType::U64 => values.push(IValue::U64(seq_reader.read_u64(store))),
            IType::F32 => values.push(IValue::F32(seq_reader.read_f32(store))),
            IType::F64 => values.push(IValue::F64(seq_reader.read_f64(store))),
            IType::String => values.push(IValue::String(read_string(store, reader, &seq_reader)?)),
            IType::ByteArray => values.push(read_byte_array(store, reader, &seq_reader)?),
            IType::Array(ty) => values.push(read_array(store, &lifter, &seq_reader, &**ty)?),
            IType::Record(record_type_id) => {
                values.push(read_record(store, lifter, &seq_reader, *record_type_id)?)
            }
        }
    }

    let record = NEVec::new(values.into_iter().collect())
        .map_err(|_| LiError::EmptyRecord(record_type.name.clone()))?;

    Ok(IValue::Record(record))
}

fn read_string<MV: MemoryView<Store>, Store: it_memory_traits::Store>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    reader: &MemoryReader<MV, Store>,
    seq_reader: &SequentialReader<'_, MV, Store>,
) -> LiResult<String> {
    let offset = seq_reader.read_u32(store);
    let size = seq_reader.read_u32(store);

    let string_mem = reader.read_raw_u8_array(store, offset, size)?;

    let string = String::from_utf8(string_mem)?;
    Ok(string)
}

fn read_byte_array<MV: MemoryView<Store>, Store: it_memory_traits::Store>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    reader: &MemoryReader<MV, Store>,
    seq_reader: &SequentialReader<'_, MV, Store>,
) -> LiResult<IValue> {
    let offset = seq_reader.read_u32(store);
    let size = seq_reader.read_u32(store);

    let array = reader.read_raw_u8_array(store, offset, size)?;

    Ok(IValue::ByteArray(array))
}

fn read_array<R: RecordResolvable, MV: MemoryView<Store>, Store: it_memory_traits::Store>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lifter: &ILifter<'_, R, MV, Store>,
    seq_reader: &SequentialReader<'_, MV, Store>,
    value_type: &IType,
) -> LiResult<IValue> {
    let offset = seq_reader.read_u32(store);
    let size = seq_reader.read_u32(store);

    super::array_lift_memory(store, lifter, value_type, offset, size)
}

fn read_record<R: RecordResolvable, MV: MemoryView<Store>, Store: it_memory_traits::Store>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lifter: &ILifter<'_, R, MV, Store>,
    seq_reader: &SequentialReader<'_, MV, Store>,
    record_type_id: u64,
) -> LiResult<IValue> {
    let offset = seq_reader.read_u32(store);

    let record_type = lifter.resolver.resolve_record(record_type_id)?;

    record_lift_memory(store, lifter, &record_type, offset)
}
