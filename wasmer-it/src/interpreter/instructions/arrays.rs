mod read_arrays;

use super::read_from_instance_mem;
use super::record_lift_memory_;
use super::write_to_instance_mem;
use read_arrays::*;

use crate::instr_error;
use crate::interpreter::instructions::to_native;
use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
    IType, IValue,
};

use std::convert::TryInto;

pub(super) fn array_lift_memory_<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
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
        IType::Boolean => read_bool_array(instance, instruction.clone(), offset, elements_count),
        IType::S8 => read_s8_array(instance, instruction.clone(), offset, elements_count),
        IType::S16 => read_s16_array(instance, instruction.clone(), offset, elements_count),
        IType::S32 => read_s32_array(instance, instruction.clone(), offset, elements_count),
        IType::S64 => read_s64_array(instance, instruction.clone(), offset, elements_count),
        IType::I32 => read_i32_array(instance, instruction.clone(), offset, elements_count),
        IType::I64 => read_i64_array(instance, instruction.clone(), offset, elements_count),
        IType::U8 => read_u8_array(instance, instruction.clone(), offset, elements_count),
        IType::U16 => read_u16_array(instance, instruction.clone(), offset, elements_count),
        IType::U32 => read_u32_array(instance, instruction.clone(), offset, elements_count),
        IType::U64 => read_u64_array(instance, instruction.clone(), offset, elements_count),
        IType::F32 => read_f32_array(instance, instruction.clone(), offset, elements_count),
        IType::F64 => read_f64_array(instance, instruction.clone(), offset, elements_count),
        IType::String => read_string_array(instance, instruction.clone(), offset, elements_count),
        IType::Record(record_type_id) => read_record_array(
            instance,
            instruction.clone(),
            *record_type_id,
            offset,
            elements_count,
        ),
        IType::ByteArray => read_array_array(
            instance,
            instruction.clone(),
            &IType::ByteArray,
            offset,
            elements_count,
        ),
        IType::Array(ty) => {
            read_array_array(instance, instruction.clone(), &ty, offset, elements_count)
        }
    }
}

pub(crate) fn array_lift_memory<Instance, Export, LocalImport, Memory, MemoryView>(
    instruction: Instruction,
    value_type: IType,
) -> crate::interpreter::ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance:
        crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    #[allow(unused_imports)]
    use crate::interpreter::stack::Stackable;
    Box::new({
        move |runtime| -> _ {
            let inputs = runtime.stack.pop(2).ok_or_else(|| {
                InstructionError::new(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            let offset: usize = to_native::<i32>(&inputs[0], instruction.clone())?
                .try_into()
                .map_err(|e| (e, "offset").into())
                .map_err(|k| InstructionError::new(instruction.clone(), k))?;

            let size: usize = to_native::<i32>(&inputs[1], instruction.clone())?
                .try_into()
                .map_err(|e| (e, "size").into())
                .map_err(|k| InstructionError::new(instruction.clone(), k))?;

            log::trace!(
                "array.lift_memory: lifting memory for value type: {:?}, popped offset {}, size {}",
                value_type,
                offset,
                size
            );

            let instance = &mut runtime.wasm_instance;
            let array = array_lift_memory_(
                *instance,
                &value_type,
                offset as _,
                size as _,
                instruction.clone(),
            )?;

            log::trace!("array.lift_memory: pushing {:?} on the stack", array);
            runtime.stack.push(array);

            Ok(())
        }
    })
}

pub(super) fn array_lower_memory_<Instance, Export, LocalImport, Memory, MemoryView>(
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
    let mut result: Vec<u64> = Vec::with_capacity(array_values.len());

    // here it's known that all interface values have the same type
    for value in array_values {
        match value {
            IValue::S8(value) => result.push(value as _),
            IValue::S16(value) => result.push(value as _),
            IValue::S32(value) => result.push(value as _),
            IValue::S64(value) => result.push(value as _),
            IValue::U8(value) => result.push(value as _),
            IValue::U16(value) => result.push(value as _),
            IValue::U32(value) => result.push(value as _),
            IValue::U64(value) => result.push(value as _),
            IValue::I32(value) => result.push(value as _),
            IValue::I64(value) => result.push(value as _),
            IValue::F32(value) => result.push(value as _),
            IValue::F64(value) => result.push(value.to_bits()),
            IValue::String(value) => {
                let string_pointer = if !value.is_empty() {
                    write_to_instance_mem(instance, instruction.clone(), value.as_bytes())?
                } else {
                    0
                };

                result.push(string_pointer as _);
                result.push(value.len() as _);
            }

            IValue::Array(values) => {
                let (array_offset, array_size) = if !values.is_empty() {
                    array_lower_memory_(instance, instruction.clone(), values)?
                } else {
                    (0, 0)
                };

                result.push(array_offset as _);
                result.push(array_size as _);
            }

            IValue::Record(values) => {
                let record_offset =
                    super::record_lower_memory_(instance, instruction.clone(), values)?;
                result.push(record_offset as _);
            }
        }
    }

    let result = safe_transmute::transmute_to_bytes::<u64>(&result);
    let result_pointer = write_to_instance_mem(instance, instruction, &result)?;

    Ok((result_pointer as _, result.len() as _))
}

pub(crate) fn array_lower_memory<Instance, Export, LocalImport, Memory, MemoryView>(
    instruction: Instruction,
    value_type: IType,
) -> crate::interpreter::ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance:
        crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    #[allow(unused_imports)]
    use crate::interpreter::stack::Stackable;
    Box::new({
        move |runtime| -> _ {
            let instance = &mut runtime.wasm_instance;
            let stack_value = runtime.stack.pop1().ok_or_else(|| {
                InstructionError::new(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            match stack_value {
                IValue::Array(values) => {
                    log::trace!("array.lower_memory: obtained {:?} values on the stack for interface type {:?}", values, value_type);

                    for value in values.iter() {
                        super::is_value_compatible_to_type(
                            &**instance,
                            &value_type,
                            &value,
                            instruction.clone(),
                        )?;
                    }

                    let (offset, size) =
                        array_lower_memory_(*instance, instruction.clone(), values)?;

                    log::trace!(
                        "array.lower_memory: pushing {}, {} on the stack",
                        offset,
                        size
                    );
                    runtime.stack.push(IValue::I32(offset as _));
                    runtime.stack.push(IValue::I32(size as _));

                    Ok(())
                }
                _ => instr_error!(
                    instruction.clone(),
                    InstructionErrorKind::InvalidValueOnTheStack {
                        expected_type: IType::Array(Box::new(value_type.clone())),
                        received_value: stack_value.clone()
                    }
                ),
            }
        }
    })
}
