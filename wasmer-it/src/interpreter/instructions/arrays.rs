use super::lilo;

use crate::errors::InstructionResult;
use crate::instr_error;
use crate::interpreter::instructions::to_native;
use crate::interpreter::stack::Stackable;
use crate::interpreter::{AsyncExecutableInstructionImpl, ExecutableInstruction, Runtime};
use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
    IType, IValue,
};
use it_lilo::lifter::ILifter;
use it_lilo::lowerer::ILowerer;
use it_lilo::lowerer::LoweredArray;
use it_lilo::traits::DEFAULT_MEMORY_INDEX;

use futures::future::BoxFuture;
use futures::FutureExt;

struct ArrayLiftMemoryAsync {
    instruction: Instruction,
    value_type: IType,
}
pub(crate) fn array_lift_memory<Instance, Export, LocalImport, Memory, MemoryView, Store>(
    instruction: Instruction,
    value_type: IType,
) -> ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView, Store>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport<Store>,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView, Store>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView<Store>,
    Instance: crate::interpreter::wasm::structures::Instance<
        Export,
        LocalImport,
        Memory,
        MemoryView,
        Store,
    >,
    Store: crate::interpreter::wasm::structures::Store,
{
    ExecutableInstruction::Async(Box::new(ArrayLiftMemoryAsync {
        instruction,
        value_type,
    }))
}

impl<Instance, Export, LocalImport, Memory, MemoryView, Store>
    AsyncExecutableInstructionImpl<Instance, Export, LocalImport, Memory, MemoryView, Store>
    for ArrayLiftMemoryAsync
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport<Store>,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView, Store>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView<Store>,
    Instance: crate::interpreter::wasm::structures::Instance<
        Export,
        LocalImport,
        Memory,
        MemoryView,
        Store,
    >,
    Store: crate::interpreter::wasm::structures::Store,
{
    fn execute<'args>(
        &'args self,
        runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>,
    ) -> BoxFuture<InstructionResult<()>> {
        async move {
            let value_type = &self.value_type;
            let instruction = &self.instruction;

            let mut inputs = runtime.stack.pop(2).ok_or_else(|| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            let offset = to_native::<i32>(inputs.remove(0), instruction.clone())? as u32;

            let size = to_native::<i32>(inputs.remove(0), instruction.clone())? as u32;

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
            let array = it_lilo::lifter::array_lift_memory(
                runtime.store,
                &lifter,
                &value_type,
                offset,
                size,
            )
            .map_err(|e| InstructionError::from_li(instruction.clone(), e))?;

            log::trace!("array.lift_memory: pushing {:?} on the stack", array);
            runtime.stack.push(array);

            Ok(())
        }
        .boxed()
    }
}

struct ArrayLowerMemoryAsync {
    instruction: Instruction,
    value_type: IType,
}

pub(crate) fn array_lower_memory<Instance, Export, LocalImport, Memory, MemoryView, Store>(
    instruction: Instruction,
    value_type: IType,
) -> ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView, Store>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport<Store>,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView, Store>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView<Store>,
    Instance: crate::interpreter::wasm::structures::Instance<
        Export,
        LocalImport,
        Memory,
        MemoryView,
        Store,
    >,
    Store: crate::interpreter::wasm::structures::Store,
{
    ExecutableInstruction::Async(Box::new(ArrayLowerMemoryAsync {
        instruction,
        value_type,
    }))
}

impl<Instance, Export, LocalImport, Memory, MemoryView, Store>
    AsyncExecutableInstructionImpl<Instance, Export, LocalImport, Memory, MemoryView, Store>
    for ArrayLowerMemoryAsync
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport<Store>,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView, Store>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView<Store>,
    Instance: crate::interpreter::wasm::structures::Instance<
        Export,
        LocalImport,
        Memory,
        MemoryView,
        Store,
    >,
    Store: crate::interpreter::wasm::structures::Store,
{
    fn execute<'args>(
        &'args self,
        runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>,
    ) -> BoxFuture<InstructionResult<()>> {
        let value_type = &self.value_type;
        let instruction = &self.instruction;

        async move {
            let instance = &mut runtime.wasm_instance;
            let stack_value = runtime.stack.pop1().ok_or_else(|| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            match stack_value {
                IValue::Array(values) => {
                    log::trace!(
                    "array.lower_memory: obtained {:?} values on the stack for interface type {:?}",
                    values,
                    value_type
                );

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

                    let mut lo_helper = lilo::LoHelper::new(&**instance);
                    let mut lowerer = ILowerer::new(memory_view, &mut lo_helper)
                        .map_err(|e| InstructionError::from_lo(instruction.clone(), e))?;

                    let LoweredArray { offset, size } =
                        it_lilo::lowerer::array_lower_memory(runtime.store, &mut lowerer, values)
                            .await
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
                    let mut lo_helper = lilo::LoHelper::new(&**instance);
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

                    let mut lowerer = ILowerer::new(memory_view, &mut lo_helper)
                        .map_err(|e| InstructionError::from_lo(instruction.clone(), e))?;

                    let offset = lowerer
                        .writer
                        .write_bytes(runtime.store, &bytearray)
                        .await
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
        .boxed()
    }
}
