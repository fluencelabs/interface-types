use super::write_to_instance_mem;
use super::LiLoResult;
use super::LiLoRecordError;
use crate::IValue;
use crate::NEVec;

use it_lilo_utils::memory_writer::MemoryWriter;

pub(crate) fn record_lower_memory_impl(
    writer: &MemoryWriter,
    values: NEVec<IValue>,
) -> LiLoResult<i32> {
    let average_field_size = 4;
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
                let offset =
                    write_to_instance_mem(instance, instruction.clone(), value.as_bytes())? as u32;

                result.extend_from_slice(&offset.to_le_bytes());
                result.extend_from_slice(&(value.len() as u32).to_le_bytes());
            }
            IValue::ByteArray(value) => {
                let array_pointer =
                    write_to_instance_mem(instance, instruction.clone(), &value)? as u32;

                result.extend_from_slice(&array_pointer.to_le_bytes());
                result.extend_from_slice(&(value.len() as u32).to_le_bytes());
            }

            IValue::Array(values) => {
                let (offset, size) =
                    super::array_lower_memory_impl(instance, instruction.clone(), values)?;

                result.extend_from_slice(&(offset as u32).to_le_bytes());
                result.extend_from_slice(&(size as u32).to_le_bytes());
            }

            IValue::Record(values) => {
                let record_ptr =
                    record_lower_memory_impl(instance, instruction.clone(), values)? as u32;

                result.extend_from_slice(&record_ptr.to_le_bytes());
            }
        }
    }

    let result_pointer = write_to_instance_mem(instance, instruction, &result)?;

    Ok(result_pointer as _)
}
