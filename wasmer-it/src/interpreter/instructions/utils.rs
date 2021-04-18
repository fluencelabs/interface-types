use crate::interpreter::instructions::ALLOCATE_FUNC_INDEX;
use crate::interpreter::wasm;
use crate::interpreter::wasm::structures::{FunctionIndex, TypedIndex};

use crate::instr_error;
use crate::interpreter::instructions::to_native;
use crate::IType;
use crate::IValue;
use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
};

pub(super) fn read_from_instance_mem<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    instruction: Instruction,
    offset: usize,
    size: usize,
) -> Result<Vec<u8>, InstructionError>
where
    Export: wasm::structures::Export + 'instance,
    LocalImport: wasm::structures::LocalImport + 'instance,
    Memory: wasm::structures::Memory<MemoryView> + 'instance,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    let memory_index = 0;
    let memory_view = instance
        .memory(memory_index)
        .ok_or_else(|| {
            InstructionError::new(
                instruction.clone(),
                InstructionErrorKind::MemoryIsMissing { memory_index },
            )
        })?
        .view();

    log::trace!("reading {} bytes from offset {}", size, offset);

    let right = offset + size;
    if right < offset || right >= memory_view.len() {
        return instr_error!(
            instruction,
            InstructionErrorKind::MemoryOutOfBoundsAccess {
                index: right,
                length: memory_view.len(),
            }
        );
    }

    Ok((&memory_view[offset..offset + size])
        .iter()
        .map(std::cell::Cell::get)
        .collect::<Vec<u8>>())
}

pub(super) fn write_to_instance_mem<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    instruction: Instruction,
    bytes: &[u8],
) -> Result<usize, InstructionError>
where
    Export: wasm::structures::Export + 'instance,
    LocalImport: wasm::structures::LocalImport + 'instance,
    Memory: wasm::structures::Memory<MemoryView> + 'instance,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    if bytes.is_empty() {
        return Ok(0);
    }

    let offset = allocate(instance, instruction.clone(), bytes.len() as _)?;

    let memory_index = 0;
    let memory_view = instance
        .memory(memory_index)
        .ok_or_else(|| {
            InstructionError::new(
                instruction.clone(),
                InstructionErrorKind::MemoryIsMissing { memory_index },
            )
        })?
        .view();

    log::trace!("writing {} bytes from offset {}", bytes.len(), offset);

    let right = offset + bytes.len();
    if right < offset || right >= memory_view.len() {
        return instr_error!(
            instruction,
            InstructionErrorKind::MemoryOutOfBoundsAccess {
                index: right,
                length: memory_view.len(),
            }
        );
    }

    for (byte_id, byte) in bytes.iter().enumerate() {
        memory_view[offset + byte_id].set(*byte);
    }

    Ok(offset)
}

pub(super) fn allocate<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    instruction: Instruction,
    size: usize,
) -> Result<usize, InstructionError>
where
    Export: wasm::structures::Export + 'instance,
    LocalImport: wasm::structures::LocalImport + 'instance,
    Memory: wasm::structures::Memory<MemoryView> + 'instance,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    let values = call_core(
        instance,
        ALLOCATE_FUNC_INDEX,
        instruction.clone(),
        vec![IValue::I32(size as _)],
    )?;
    if values.len() != 1 {
        return instr_error!(
            instruction,
            InstructionErrorKind::LocalOrImportSignatureMismatch {
                function_index: ALLOCATE_FUNC_INDEX,
                expected: (vec![IType::I32], vec![]),
                received: (vec![], vec![]),
            }
        );
    }
    to_native::<i32>(&values[0], instruction).map(|v| v as usize)
}

fn call_core<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    function_index: u32,
    instruction: Instruction,
    inputs: Vec<IValue>,
) -> Result<Vec<IValue>, InstructionError>
where
    Export: wasm::structures::Export + 'instance,
    LocalImport: wasm::structures::LocalImport + 'instance,
    Memory: wasm::structures::Memory<MemoryView> + 'instance,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    let index = FunctionIndex::new(function_index as usize);
    let local_or_import = instance.local_or_import(index).ok_or_else(|| {
        InstructionError::new(
            instruction.clone(),
            InstructionErrorKind::LocalOrImportIsMissing { function_index },
        )
    })?;

    crate::interpreter::instructions::check_function_signature(
        instance,
        local_or_import,
        &inputs,
        instruction.clone(),
    )?;

    let outputs = local_or_import.call(&inputs).map_err(|_| {
        InstructionError::new(
            instruction.clone(),
            InstructionErrorKind::LocalOrImportCall {
                function_name: local_or_import.name().to_string(),
            },
        )
    })?;

    Ok(outputs)
}
