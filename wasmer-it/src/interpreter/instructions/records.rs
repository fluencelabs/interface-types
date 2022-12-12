use super::lilo;
use crate::instr_error;
use crate::interpreter::instructions::{is_record_fields_compatible_to_type, to_native};
use crate::IType;
use crate::IValue;
use crate::{errors::InstructionError, errors::InstructionErrorKind, interpreter::Instruction};

use it_lilo::lifter::ILifter;
use it_lilo::lowerer::ILowerer;
use it_lilo::traits::DEFAULT_MEMORY_INDEX;

pub(crate) fn record_lift_memory<Instance, Export, LocalImport, Memory, MemoryView, Store>(
    record_type_id: u64,
    instruction: Instruction,
) -> crate::interpreter::ExecutableInstruction<
    Instance,
    Export,
    LocalImport,
    Memory,
    MemoryView,
    Store,
>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport<Store>,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<
        Export,
        LocalImport,
        Memory,
        MemoryView,
        Store,
    >,
    Store: crate::interpreter::wasm::structures::Store,
{
    #[allow(unused_imports)]
    use crate::interpreter::stack::Stackable;
    Box::new({
        move |runtime| -> _ {
            let mut inputs = runtime.stack.pop(1).ok_or_else(|| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            let offset: u32 = to_native::<i32>(inputs.remove(0), instruction.clone())? as u32;

            // TODO: size = 0
            let instance = &runtime.wasm_instance;
            let record_type = instance.wit_record_by_id(record_type_id).ok_or_else(|| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::RecordTypeByNameIsMissing { record_type_id },
                )
            })?;

            log::trace!(
                "record.lift_memory: record {:?} resolved for type id {}",
                record_type,
                record_type_id
            );

            let memory_index = DEFAULT_MEMORY_INDEX;
            let memory_view = instance
                .memory(memory_index)
                .ok_or_else(|| {
                    InstructionError::from_error_kind(
                        instruction.clone(),
                        InstructionErrorKind::MemoryIsMissing { memory_index },
                    )
                })?
                .view();

            let li_helper = lilo::LiHelper::new(&**instance);
            let lifter = ILifter::new(memory_view, &li_helper);
            let record = it_lilo::lifter::record_lift_memory(&lifter, record_type, offset)
                .map_err(|e| InstructionError::from_li(instruction.clone(), e))?;

            log::debug!("record.lift_memory: pushing {:?} on the stack", record);
            runtime.stack.push(record);

            Ok(())
        }
    })
}

pub(crate) fn record_lower_memory<Instance, Export, LocalImport, Memory, MemoryView, Store>(
    record_type_id: u64,
    instruction: Instruction,
) -> crate::interpreter::ExecutableInstruction<
    Instance,
    Export,
    LocalImport,
    Memory,
    MemoryView,
    Store,
>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport<Store>,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<
        Export,
        LocalImport,
        Memory,
        MemoryView,
        Store,
    >,
    Store: crate::interpreter::wasm::structures::Store,
{
    #[allow(unused_imports)]
    use crate::interpreter::stack::Stackable;
    Box::new({
        move |runtime| -> _ {
            let instance = &mut runtime.wasm_instance;

            match runtime.stack.pop1() {
                Some(IValue::Record(record_fields)) => {
                    is_record_fields_compatible_to_type(
                        &**instance,
                        record_type_id,
                        &record_fields,
                    )
                    .map_err(|e| InstructionError::from_error_kind(instruction.clone(), e))?;

                    log::debug!("record.lower_memory: obtained {:?} values on the stack for record type = {}", record_fields, record_type_id);

                    let memory_index = DEFAULT_MEMORY_INDEX;
                    let memory_view = instance
                        .memory(memory_index)
                        .ok_or_else(|| {
                            InstructionError::from_error_kind(
                                instruction.clone(),
                                InstructionErrorKind::MemoryIsMissing { memory_index },
                            )
                        })?
                        .view();

                    let mut lo_helper = lilo::LoHelper::new(&**instance, runtime.wasm_store);
                    let mut memory_writer = ILowerer::new(memory_view, &mut lo_helper)
                        .map_err(|e| InstructionError::from_lo(instruction.clone(), e))?;
                    let offset =
                        it_lilo::lowerer::record_lower_memory(&mut memory_writer, record_fields)
                            .map_err(|e| InstructionError::from_lo(instruction.clone(), e))?;

                    log::debug!("record.lower_memory: pushing {} on the stack", offset);
                    runtime.stack.push(IValue::I32(offset as i32));

                    Ok(())
                }
                Some(value) => instr_error!(
                    instruction.clone(),
                    InstructionErrorKind::InvalidValueOnTheStack {
                        expected_type: IType::Record(record_type_id),
                        received_value: value,
                    }
                ),
                None => instr_error!(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 }
                ),
            }
        }
    })
}
