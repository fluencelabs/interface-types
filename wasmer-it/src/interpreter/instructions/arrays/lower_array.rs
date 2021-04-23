use crate::IValue;

use it_lilo_utils::memory_writer::Heapable;
use it_lilo_utils::memory_writer::MemoryWriter;
use it_lilo_utils::ser_value_size;
use it_lilo_utils::type_tag_form_ivalue;
use it_lilo_utils::WriteResult;

pub(crate) fn array_lower_memory_impl<T: Heapable>(
    writer: &MemoryWriter<T>,
    array_values: Vec<IValue>,
) -> WriteResult<(usize, usize)> {
    if array_values.is_empty() {
        return Ok((0, 0));
    }

    let elements_count = array_values.len() as u32;
    let size = ser_value_size(&array_values[0]) * elements_count;
    let type_tag = type_tag_form_ivalue(&array_values[0]);
    let seq_writer = writer.sequential_writer(size, type_tag)?;

    // here it's known that all interface values have the same type
    for value in array_values {
        match value {
            IValue::Boolean(value) => seq_writer.write_u8(writer, value as _),
            IValue::S8(value) => seq_writer.write_u8(writer, value as _),
            IValue::S16(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::S32(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::S64(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::U8(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::U16(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::U32(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::U64(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::U128(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::I32(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::I64(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::F32(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::F64(value) => seq_writer.write_array(writer, value.to_le_bytes()),
            IValue::String(value) => {
                let offset = writer.write_bytes(value.as_bytes())? as u32;

                seq_writer.write_array(writer, offset.to_le_bytes());
                seq_writer.write_array(writer, (value.len() as u32).to_le_bytes());
            }
            IValue::ByteArray(values) => {
                let offset = writer.write_bytes(&values)? as u32;

                seq_writer.write_array(writer, offset.to_le_bytes());
                seq_writer.write_array(writer, (values.len() as u32).to_le_bytes());
            }
            IValue::Array(values) => {
                let (offset, size) = array_lower_memory_impl(writer, values)?;

                seq_writer.write_array(writer, (offset as u32).to_le_bytes());
                seq_writer.write_array(writer, (size as u32).to_le_bytes());
            }
            IValue::Record(values) => {
                let offset = super::record_lower_memory_impl(writer, values)? as u32;
                seq_writer.write_array(writer, offset.to_le_bytes());
            }
        }
    }

    let offset = seq_writer.start_offset();
    Ok((offset as _, elements_count as _))
}
