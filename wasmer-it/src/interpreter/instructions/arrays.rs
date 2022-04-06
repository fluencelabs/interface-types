use super::lilo;

use crate::instr_error;
use crate::interpreter::instructions::to_native;
use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
    IType, IValue,
};
use it_lilo::lifter::ILifter;
use it_lilo::lowerer::ILowerer;
use it_lilo::lowerer::LoweredArray;
use it_lilo::traits::DEFAULT_MEMORY_INDEX;

use std::convert::TryInto;

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
            let mut inputs = runtime.stack.pop(2).ok_or_else(|| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            let offset: u32 = to_native::<i32>(inputs.remove(0), instruction.clone())?
                .try_into()
                .map_err(|e| (e, "offset").into())
                .map_err(|k| InstructionError::from_error_kind(instruction.clone(), k))?;

            let size: u32 = to_native::<i32>(inputs.remove(0), instruction.clone())?
                .try_into()
                .map_err(|e| (e, "size").into())
                .map_err(|k| InstructionError::from_error_kind(instruction.clone(), k))?;

            log::trace!(
                "array.lift_memory: lifting memory for value type: {:?}, popped offset {}, size {}",
                value_type,
                offset,
                size
            );

            let instance = &mut runtime.wasm_instance;

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
            let array = it_lilo::lifter::array_lift_memory(&lifter, &value_type, offset, size)
                .map_err(|e| InstructionError::from_li(instruction.clone(), e))?;

            log::trace!("array.lift_memory: pushing {:?} on the stack", array);
            runtime.stack.push(array);

            Ok(())
        }
    })
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
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            match stack_value {
                IValue::Array(values) => {
                    log::trace!("array.lower_memory: obtained {:?} values on the stack for interface type {:?}", values, value_type);

                    for value in values.iter() {
                        super::is_value_compatible_to_type(&**instance, &value_type, &value)
                            .map_err(|e| {
                                InstructionError::from_error_kind(instruction.clone(), e)
                            })?;
                    }
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

                    let lo_helper = lilo::LoHelper::new(&**instance);
                    let lowerer = ILowerer::new(memory_view, &lo_helper)
                        .map_err(|e| InstructionError::from_lo(instruction.clone(), e))?;

                    let LoweredArray { offset, size } =
                        it_lilo::lowerer::array_lower_memory(&lowerer, values)
                            .map_err(|e| InstructionError::from_lo(instruction.clone(), e))?;

                    log::trace!(
                        "array.lower_memory: pushing {}, {} on the stack",
                        offset,
                        size
                    );
                    runtime.stack.push(IValue::I32(offset as _));
                    runtime.stack.push(IValue::I32(size as _));

                    Ok(())
                }
                IValue::ByteArray(bytearray) => {
                    let lo_helper = lilo::LoHelper::new(&**instance);
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

                    let lowerer = ILowerer::new(memory_view, &lo_helper)
                        .map_err(|e| InstructionError::from_lo(instruction.clone(), e))?;

                    let offset = lowerer
                        .writer
                        .write_bytes(&bytearray)
                        .map_err(|e| InstructionError::from_lo(instruction.clone(), e))?;
                    let size = bytearray.len();

                    log::trace!(
                        "array.lower_memory: pushing bytes {}, {} on the stack",
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
