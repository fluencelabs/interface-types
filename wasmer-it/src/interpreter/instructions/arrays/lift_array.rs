use super::lilo::*;

use crate::IType;
use crate::IValue;

use crate::interpreter::instructions::record_lift_memory_impl;
use it_lilo_utils::ser_type_size;

pub(crate) fn array_lift_memory_impl(
    li_helper: &LiHelper,
    value_type: &IType,
    offset: usize,
    elements_count: usize,
) -> LiLoResult<IValue> {
    if elements_count == 0 {
        return Ok(IValue::Array(vec![]));
    }

    let reader = &li_helper.reader;

    let ivalues = match value_type {
        IType::Boolean => reader.read_bool_array(offset, elements_count)?,
        IType::S8 => reader.read_s8_array(offset, elements_count)?,
        IType::S16 => reader.read_s16_array(offset, elements_count)?,
        IType::S32 => reader.read_s32_array(offset, elements_count)?,
        IType::S64 => reader.read_s64_array(offset, elements_count)?,
        IType::I32 => reader.read_i32_array(offset, elements_count)?,
        IType::I64 => reader.read_i64_array(offset, elements_count)?,
        IType::U8 => reader.read_u8_array(offset, elements_count)?,
        IType::U16 => reader.read_u16_array(offset, elements_count)?,
        IType::U32 => reader.read_u32_array(offset, elements_count)?,
        IType::U64 => reader.read_u64_array(offset, elements_count)?,
        IType::F32 => reader.read_f32_array(offset, elements_count)?,
        IType::F64 => reader.read_f64_array(offset, elements_count)?,
        IType::String => read_string_array(li_helper, offset, elements_count)?,
        IType::ByteArray => read_array_array(li_helper, &IType::ByteArray, offset, elements_count)?,
        IType::Array(ty) => read_array_array(li_helper, &ty, offset, elements_count)?,
        IType::Record(record_type_id) => {
            read_record_array(li_helper, *record_type_id, offset, elements_count)?
        }
    };

    Ok(IValue::Array(ivalues))
}

fn read_string_array(
    li_helper: &LiHelper,
    offset: usize,
    elements_count: usize,
) -> LiLoResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count);
    let seq_reader = li_helper
        .reader
        .sequential_reader(offset, ser_type_size(&IType::String) * elements_count)?;

    for _ in 0..elements_count {
        let offset = seq_reader.read_u32();
        let size = seq_reader.read_u32();

        let raw_str = li_helper.reader.read_raw_u8_array(offset as _, size as _)?;
        let str = String::from_utf8(raw_str)?;
        result.push(IValue::String(str));
    }

    Ok(result)
}

fn read_array_array(
    li_helper: &LiHelper,
    ty: &IType,
    offset: usize,
    elements_count: usize,
) -> LiLoResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count);
    let seq_reader = li_helper
        .reader
        .sequential_reader(offset, ser_type_size(ty) * elements_count)?;

    for _ in 0..elements_count {
        let offset = seq_reader.read_u32();
        let size = seq_reader.read_u32();

        let array = array_lift_memory_impl(li_helper, ty, offset as _, size as _)?;
        result.push(array);
    }

    Ok(result)
}

fn read_record_array(
    li_helper: &LiHelper,
    record_type_id: u64,
    offset: usize,
    elements_count: usize,
) -> LiLoResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count);
    let seq_reader = li_helper
        .reader
        .sequential_reader(offset, ser_type_size(&IType::Record(0)) * elements_count)?;

    for _ in 0..elements_count {
        let offset = seq_reader.read_u32();
        let record_ty = (li_helper.record_resolver)(record_type_id)?;

        let record = record_lift_memory_impl(li_helper, &record_ty, offset as _)?;
        result.push(record);
    }

    Ok(result)
}
