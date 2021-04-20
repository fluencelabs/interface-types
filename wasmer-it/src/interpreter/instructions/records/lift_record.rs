use super::lilo::*;

use crate::IRecordType;
use crate::IType;
use crate::IValue;
use crate::NEVec;

use it_lilo_utils::memory_reader::MemoryReader;
use it_lilo_utils::memory_reader::SequentialReader;
use it_lilo_utils::record_size;

#[rustfmt::skip]
pub(crate) fn record_lift_memory_impl(
    li_helper: &LiHelper<'_>,
    record_type: &IRecordType,
    offset: usize,
) -> LiLoResult<IValue> {
    let mut values = Vec::with_capacity(record_type.fields.len());

    let size = record_size(record_type);
    let reader = &li_helper.reader;
    let seq_reader = reader.sequential_reader(offset, size)?;

    for field in (*record_type.fields).iter() {
        match &field.ty {
            IType::Boolean => values.push(IValue::Boolean(seq_reader.read_u8() != 0)),
            IType::S8 => values.push(IValue::S8(seq_reader.read_i8())),
            IType::S16 => values.push(IValue::S16(seq_reader.read_i16())),
            IType::S32 => values.push(IValue::S32(seq_reader.read_i32())),
            IType::S64 => values.push(IValue::S64(seq_reader.read_i64())),
            IType::I32 => values.push(IValue::I32(seq_reader.read_i32())),
            IType::I64 => values.push(IValue::I64(seq_reader.read_i64())),
            IType::U8 => values.push(IValue::U8(seq_reader.read_u8())),
            IType::U16 => values.push(IValue::U16(seq_reader.read_u16())),
            IType::U32 => values.push(IValue::U32(seq_reader.read_u32())),
            IType::U64 => values.push(IValue::U64(seq_reader.read_u64())),
            IType::F32 => values.push(IValue::F32(seq_reader.read_f32())),
            IType::F64 => values.push(IValue::F64(seq_reader.read_f64())),
            IType::String => values.push(IValue::String(read_string(reader, &seq_reader)?)),
            IType::ByteArray => values.push(read_byte_array(reader, &seq_reader)?),
            IType::Array(ty) =>  values.push(read_array(&li_helper, &seq_reader, &**ty)?),
            IType::Record(record_type_id) => values.push(read_record(li_helper, &seq_reader, *record_type_id)?),
        }
    }

    let record = NEVec::new(values.into_iter().collect())
        .map_err(|_| LiLoError::EmptyRecord(record_type.name.clone()))?;

    Ok(IValue::Record(record))
}

fn read_string(reader: &MemoryReader, seq_reader: &SequentialReader) -> LiLoResult<String> {
    let offset = seq_reader.read_u32();
    let size = seq_reader.read_u32();

    let string_mem = reader.read_raw_u8_array(offset as _, size as _)?;

    let string = String::from_utf8(string_mem)?;
    Ok(string)
}

fn read_byte_array(reader: &MemoryReader, seq_reader: &SequentialReader) -> LiLoResult<IValue> {
    let offset = seq_reader.read_u32();
    let size = seq_reader.read_u32();

    let array = reader.read_raw_u8_array(offset as _, size as _)?;

    Ok(IValue::ByteArray(array))
}

fn read_array(
    li_helper: &LiHelper,
    seq_reader: &SequentialReader,
    value_type: &IType,
) -> LiLoResult<IValue> {
    let offset = seq_reader.read_u32();
    let size = seq_reader.read_u32();

    super::array_lift_memory_impl(li_helper, value_type, offset as _, size as _)
}

fn read_record(
    li_helper: &LiHelper,
    seq_reader: &SequentialReader,
    record_type_id: u64,
) -> LiLoResult<IValue> {
    let offset = seq_reader.read_u32();

    let record_type = (li_helper.record_resolver)(record_type_id)?;

    record_lift_memory_impl(li_helper, &record_type, offset as _)
}
