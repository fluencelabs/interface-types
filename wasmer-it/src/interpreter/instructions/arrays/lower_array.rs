use super::utils::MemoryWriter;
use super::write_to_instance_mem;

use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
    IValue,
};

pub(crate) fn array_lower_memory_impl<Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &mut Instance,
    instruction: Instruction,
    array_values: Vec<IValue>,
) -> Result<(usize, usize), InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance:
        crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    if array_values.is_empty() {
        return Ok((0, 0));
    }

    let size_to_allocate = value_size(&array_values[0]) * array_values.len();
    let offset = super::allocate(instance, instruction.clone(), size_to_allocate)?;

    let memory_index = 0;
    let memory_view = &instance
        .memory(memory_index)
        .ok_or_else(|| {
            InstructionError::new(
                instruction.clone(),
                InstructionErrorKind::MemoryIsMissing { memory_index },
            )
        })?
        .view();
    let writer = MemoryWriter::new(memory_view, offset);

    let values_count = array_values.len();

    // here it's known that all interface values have the same type
    for value in array_values {
        match value {
            IValue::Boolean(value) => writer.write_u8(value as _),
            IValue::S8(value) => writer.write_u8(value as _),
            IValue::S16(value) => writer.write_array(value.to_le_bytes()),
            IValue::S32(value) => writer.write_array(value.to_le_bytes()),
            IValue::S64(value) => writer.write_array(value.to_le_bytes()),
            IValue::U8(value) => writer.write_array(value.to_le_bytes()),
            IValue::U16(value) => writer.write_array(value.to_le_bytes()),
            IValue::U32(value) => writer.write_array(value.to_le_bytes()),
            IValue::U64(value) => writer.write_array(value.to_le_bytes()),
            IValue::I32(value) => writer.write_array(value.to_le_bytes()),
            IValue::I64(value) => writer.write_array(value.to_le_bytes()),
            IValue::F32(value) => writer.write_array(value.to_le_bytes()),
            IValue::F64(value) => writer.write_array(value.to_le_bytes()),
            IValue::String(value) => {
                let string_pointer =
                    write_to_instance_mem(instance, instruction.clone(), value.as_bytes())?;

                writer.write_array(string_pointer.to_le_bytes());
                writer.write_array(value.len().to_le_bytes());
            }
            IValue::ByteArray(values) => writer.write_slice(&values),
            IValue::Array(values) => {
                let (array_offset, array_size) =
                    array_lower_memory_impl(instance, instruction.clone(), values)?;

                writer.write_array(array_offset.to_le_bytes());
                writer.write_array(array_size.to_le_bytes());
            }

            IValue::Record(values) => {
                let record_offset =
                    super::record_lower_memory_(instance, instruction.clone(), values)?;
                writer.write_array(record_offset.to_le_bytes());
            }
        }
    }

    Ok((offset as _, values_count as _))
}

fn value_size(value: &IValue) -> usize {
    match value {
        IValue::Boolean(_) => 1,
        IValue::S8(_) => 1,
        IValue::S16(_) => 2,
        IValue::S32(_) => 4,
        IValue::S64(_) => 8,
        IValue::U8(_) => 1,
        IValue::U16(_) => 2,
        IValue::U32(_) => 4,
        IValue::U64(_) => 8,
        IValue::F32(_) => 4,
        IValue::F64(_) => 8,
        IValue::String(_) => 4,
        IValue::ByteArray(_) => 4,
        IValue::Array(_) => 4,
        IValue::I32(_) => 4,
        IValue::I64(_) => 8,
        IValue::Record(_) => 4,
    }
}
