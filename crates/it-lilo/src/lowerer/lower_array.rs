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
use crate::traits::Allocatable;
use crate::utils::ser_value_size;
use crate::utils::type_tag_form_ivalue;
use crate::IValue;

use it_memory_traits::MemoryView;

pub struct LoweredArray {
    pub offset: u32,
    pub size: u32,
}

impl LoweredArray {
    pub fn new(offset: u32, size: u32) -> Self {
        Self { offset, size }
    }

    pub fn empty() -> Self {
        Self { offset: 0, size: 0 }
    }
}

#[async_recursion::async_recursion]
pub async fn array_lower_memory<
    A: Allocatable<MV, Store>,
    MV: MemoryView<Store>,
    Store: it_memory_traits::Store,
>(
    store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
    lowerer: &mut ILowerer<'_, A, MV, Store>,
    array_values: Vec<IValue>,
) -> LoResult<LoweredArray> {
    if array_values.is_empty() {
        return Ok(LoweredArray::empty());
    }

    let elements_count = array_values.len() as u32;
    let size = ser_value_size(&array_values[0]) * elements_count;
    let type_tag = type_tag_form_ivalue(&array_values[0]);
    let seq_writer = lowerer
        .writer
        .sequential_writer(store, size, type_tag)
        .await?;

    // here it's known that all interface values have the same type
    for value in array_values {
        match value {
            IValue::Boolean(value) => seq_writer.write_u8(store, &lowerer.writer, value as _),
            IValue::S8(value) => seq_writer.write_u8(store, &lowerer.writer, value as _),
            IValue::S16(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::S32(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::S64(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::U8(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::U16(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::U32(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::U64(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::I32(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::I64(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::F32(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::F64(value) => {
                seq_writer.write_bytes(store, &lowerer.writer, &value.to_le_bytes())
            }
            IValue::String(value) => {
                let offset = lowerer.writer.write_bytes(store, &value.as_bytes()).await? as u32;

                seq_writer.write_bytes(store, &lowerer.writer, &offset.to_le_bytes());
                seq_writer.write_bytes(store, &lowerer.writer, &(value.len() as u32).to_le_bytes());
            }
            IValue::ByteArray(values) => {
                let offset = lowerer.writer.write_bytes(store, &values).await? as u32;

                seq_writer.write_bytes(store, &lowerer.writer, &offset.to_le_bytes());
                seq_writer.write_bytes(
                    store,
                    &lowerer.writer,
                    &(values.len() as u32).to_le_bytes(),
                );
            }
            IValue::Array(values) => {
                let LoweredArray { offset, size } =
                    array_lower_memory(store, lowerer, values).await?;

                seq_writer.write_bytes(store, &lowerer.writer, &(offset as u32).to_le_bytes());
                seq_writer.write_bytes(store, &lowerer.writer, &(size as u32).to_le_bytes());
            }
            IValue::Record(values) => {
                let offset = super::record_lower_memory(store, lowerer, values).await? as u32;
                seq_writer.write_bytes(store, &lowerer.writer, &offset.to_le_bytes());
            }
        }
    }

    let offset = seq_writer.start_offset();
    let lowered_array = LoweredArray::new(offset, elements_count);
    Ok(lowered_array)
}
