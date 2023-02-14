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

use super::ILowerer;
use super::LoResult;
use super::LoweredArray;
use crate::traits::Allocatable;
use crate::IValue;
use crate::NEVec;

use it_memory_traits::MemoryView;

pub fn record_lower_memory<
    A: Allocatable<MV, Store>,
    MV: MemoryView<Store>,
    Store: it_memory_traits::Store,
>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lowerer: &mut ILowerer<'_, A, MV, Store>,
    values: NEVec<IValue>,
) -> LoResult<u32> {
    let average_field_size = 4;
    // TODO: avoid this additional allocation after fixing github.com/fluencelabs/fce/issues/77
    let mut result: Vec<u8> = Vec::with_capacity(average_field_size * values.len());

    for value in values.into_vec() {
        match value {
            IValue::Boolean(value) => result.push(value as _),
            IValue::S8(value) => result.push(value as _),
            IValue::S16(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::S32(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::S64(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::U8(value) => result.push(value),
            IValue::U16(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::U32(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::U64(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::I32(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::I64(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::F32(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::F64(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::String(value) => {
                let offset = lowerer.writer.write_bytes(store, value.as_bytes())?;

                result.extend_from_slice(&offset.to_le_bytes());
                result.extend_from_slice(&(value.len() as u32).to_le_bytes());
            }
            IValue::ByteArray(value) => {
                let offset = lowerer.writer.write_bytes(store, &value)?;

                result.extend_from_slice(&offset.to_le_bytes());
                result.extend_from_slice(&(value.len() as u32).to_le_bytes());
            }

            IValue::Array(values) => {
                let LoweredArray { offset, size } =
                    super::array_lower_memory(store, lowerer, values)?;

                result.extend_from_slice(&(offset).to_le_bytes());
                result.extend_from_slice(&(size).to_le_bytes());
            }

            IValue::Record(values) => {
                let offset = record_lower_memory(store, lowerer, values)?;

                result.extend_from_slice(&offset.to_le_bytes());
            }
        }
    }

    let result_pointer = lowerer.writer.write_bytes(store, &result)?;

    Ok(result_pointer)
}
