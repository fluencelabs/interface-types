use super::lilo::*;

use crate::IValue;
use crate::NEVec;

pub(crate) fn record_lower_memory_impl(
    lo_helper: &LoHelper,
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
                let offset = lo_helper.write_to_mem(value.as_bytes())? as u32;

                result.extend_from_slice(&offset.to_le_bytes());
                result.extend_from_slice(&(value.len() as u32).to_le_bytes());
            }
            IValue::ByteArray(value) => {
                let offset = lo_helper.write_to_mem(&value)? as u32;

                result.extend_from_slice(&offset.to_le_bytes());
                result.extend_from_slice(&(value.len() as u32).to_le_bytes());
            }

            IValue::Array(values) => {
                let (offset, size) = super::array_lower_memory_impl(lo_helper, values)?;

                result.extend_from_slice(&(offset as u32).to_le_bytes());
                result.extend_from_slice(&(size as u32).to_le_bytes());
            }

            IValue::Record(values) => {
                let offset = record_lower_memory_impl(lo_helper, values)? as u32;

                result.extend_from_slice(&offset.to_le_bytes());
            }
        }
    }

    let result_pointer = lo_helper.write_to_mem(&result)?;

    Ok(result_pointer as _)
}
