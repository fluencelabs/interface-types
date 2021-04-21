use super::lilo::*;

use crate::IValue;

use it_lilo_utils::ser_value_size;
use it_lilo_utils::type_tag_form_ivalue;

pub(crate) fn array_lower_memory_impl(
    lo_helper: &LoHelper,
    array_values: Vec<IValue>,
) -> LiLoResult<(usize, usize)> {
    if array_values.is_empty() {
        return Ok((0, 0));
    }

    let elements_count = array_values.len();
    let size_to_allocate = ser_value_size(&array_values[0]) * elements_count;
    let offset = (lo_helper.allocate)(
        size_to_allocate as _,
        type_tag_form_ivalue(&array_values[0]) as _,
    )?;

    let seq_writer = lo_helper
        .writer
        .sequential_writer(offset, size_to_allocate)?;

    // here it's known that all interface values have the same type
    for value in array_values {
        match value {
            IValue::Boolean(value) => seq_writer.write_u8(value as _),
            IValue::S8(value) => seq_writer.write_u8(value as _),
            IValue::S16(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::S32(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::S64(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::U8(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::U16(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::U32(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::U64(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::I32(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::I64(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::F32(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::F64(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::String(value) => {
                let offset = lo_helper.write_to_mem(value.as_bytes())? as u32;

                seq_writer.write_array(offset.to_le_bytes());
                seq_writer.write_array((value.len() as u32).to_le_bytes());
            }
            IValue::ByteArray(values) => {
                let offset = lo_helper.write_to_mem(&values)? as u32;

                seq_writer.write_array(offset.to_le_bytes());
                seq_writer.write_array((values.len() as u32).to_le_bytes());
            }
            IValue::Array(values) => {
                let (offset, size) = array_lower_memory_impl(lo_helper, values)?;

                seq_writer.write_array((offset as u32).to_le_bytes());
                seq_writer.write_array((size as u32).to_le_bytes());
            }

            IValue::Record(values) => {
                let offset = super::record_lower_memory_impl(lo_helper, values)? as u32;
                seq_writer.write_array(offset.to_le_bytes());
            }
        }
    }

    Ok((offset as _, elements_count as _))
}
