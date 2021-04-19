use super::read_arrays::*;

use crate::{errors::InstructionError, interpreter::Instruction, IType, IValue};

pub(crate) fn array_lift_memory_impl<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    value_type: &IType,
    offset: usize,
    elements_count: usize,
    instruction: Instruction,
) -> Result<IValue, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    if elements_count == 0 {
        return Ok(IValue::Array(vec![]));
    }

    match value_type {
        IType::Boolean => read_bool_array(instance, instruction, offset, elements_count),
        IType::S8 => read_s8_array(instance, instruction, offset, elements_count),
        IType::S16 => read_s16_array(instance, instruction, offset, elements_count),
        IType::S32 => read_s32_array(instance, instruction, offset, elements_count),
        IType::S64 => read_s64_array(instance, instruction, offset, elements_count),
        IType::I32 => read_i32_array(instance, instruction, offset, elements_count),
        IType::I64 => read_i64_array(instance, instruction, offset, elements_count),
        IType::U8 => read_u8_array(instance, instruction, offset, elements_count),
        IType::U16 => read_u16_array(instance, instruction, offset, elements_count),
        IType::U32 => read_u32_array(instance, instruction, offset, elements_count),
        IType::U64 => read_u64_array(instance, instruction, offset, elements_count),
        IType::F32 => read_f32_array(instance, instruction, offset, elements_count),
        IType::F64 => read_f64_array(instance, instruction, offset, elements_count),
        IType::String => read_string_array(instance, instruction, offset, elements_count),
        IType::Record(record_type_id) => read_record_array(
            instance,
            instruction,
            *record_type_id,
            offset,
            elements_count,
        ),
        IType::ByteArray => read_array_array(
            instance,
            instruction,
            &IType::ByteArray,
            offset,
            elements_count,
        ),
        IType::Array(ty) => read_array_array(instance, instruction, &ty, offset, elements_count),
    }
}
