mod lift_record;
mod lower_record;
mod value_reader;

pub use lift_record::record_size;

pub(crate) use lift_record::record_lift_memory_impl;
pub(crate) use lower_record::record_lower_memory_impl;

use super::array_lift_memory_impl;
use super::array_lower_memory_impl;
use super::read_from_instance_mem;
use super::write_to_instance_mem;

use crate::instr_error;
use crate::interpreter::instructions::{is_record_fields_compatible_to_type, to_native};
use crate::IType;
use crate::IValue;
use crate::{errors::InstructionError, errors::InstructionErrorKind, interpreter::Instruction};

use std::convert::TryInto;

pub(crate) fn record_lift_memory<Instance, Export, LocalImport, Memory, MemoryView>(
    record_type_id: u64,
    instruction: Instruction,
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
            let inputs = runtime.stack.pop(1).ok_or_else(|| {
                InstructionError::new(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            let offset: usize = to_native::<i32>(&inputs[0], instruction.clone())?
                .try_into()
                .map_err(|e| (e, "offset").into())
                .map_err(|k| InstructionError::new(instruction.clone(), k))?;

            // TODO: size = 0
            let instance = &runtime.wasm_instance;
            let record_type = instance.wit_record_by_id(record_type_id).ok_or_else(|| {
                InstructionError::new(
                    instruction.clone(),
                    InstructionErrorKind::RecordTypeByNameIsMissing { record_type_id },
                )
            })?;

            log::trace!(
                "record.lift_memory: record {:?} resolved for type id {}",
                record_type,
                record_type_id
            );

            let record =
                record_lift_memory_impl(&**instance, record_type, offset, instruction.clone())?;

            log::debug!("record.lift_memory: pushing {:?} on the stack", record);
            runtime.stack.push(record);

            Ok(())
        }
    })
}

pub(crate) fn record_lower_memory<Instance, Export, LocalImport, Memory, MemoryView>(
    record_type_id: u64,
    instruction: Instruction,
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

            match runtime.stack.pop1() {
                Some(IValue::Record(record_fields)) => {
                    is_record_fields_compatible_to_type(
                        &**instance,
                        record_type_id,
                        &record_fields,
                        instruction.clone(),
                    )?;

                    log::debug!("record.lower_memory: obtained {:?} values on the stack for record type = {}", record_fields, record_type_id);

                    let offset =
                        record_lower_memory_impl(*instance, instruction.clone(), record_fields)?;

                    log::debug!("record.lower_memory: pushing {} on the stack", offset);
                    runtime.stack.push(IValue::I32(offset));

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
