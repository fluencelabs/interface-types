use super::to_native;
use crate::instr_error;
use crate::IType;
use crate::IValue;
use crate::{
    errors::{InstructionError, InstructionErrorKind, InstructionResult},
    interpreter::stack::Stackable,
    interpreter::Instruction,
    interpreter::Runtime,
};

use it_lilo::traits::DEFAULT_MEMORY_INDEX;

use futures::future::BoxFuture;
use futures::FutureExt;

struct ByteArrayLiftMemoryAsync {
    instruction: Instruction,
}

impl_async_executable_instruction!(
    byte_array_lift_memory(instruction: Instruction) -> _ {
        Box::new(ByteArrayLiftMemoryAsync{instruction})
    }
    ByteArrayLiftMemoryAsync {
        fn execute<'args>(&'args self, runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>)
        -> BoxFuture<InstructionResult<()>> {
            async move {
                let mut inputs = runtime.stack.pop(2).ok_or_else(|| {
                    InstructionError::from_error_kind(
                        self.instruction.clone(),
                        InstructionErrorKind::StackIsTooSmall { needed: 2 },
                    )
                })?;

                let memory_index = DEFAULT_MEMORY_INDEX;
                let memory = runtime
                    .wasm_instance
                    .memory(memory_index)
                    .ok_or_else(|| {
                        InstructionError::from_error_kind(
                            self.instruction.clone(),
                            InstructionErrorKind::MemoryIsMissing { memory_index },
                        )
                    })?;

                let pointer = to_native::<i32>(inputs.remove(0), self.instruction.clone())? as u32;
                let length = to_native::<i32>(inputs.remove(0), self.instruction.clone())? as u32;

                let memory_view = memory.view();

                if length == 0 {
                    runtime.stack.push(IValue::ByteArray(vec![]));

                    return Ok(())
                }

                memory_view
                    .check_bounds(runtime.store, pointer, length)
                    .map_err(|e| InstructionError::from_memory_access(self.instruction.clone(), e))?;

                let data = memory_view.read_vec(runtime.store, pointer, length);

                log::debug!("byte_array.lift_memory: pushing {:?} on the stack", data);
                runtime.stack.push(IValue::ByteArray(data));

                Ok(())
            }.boxed()
        }
    }
);

struct ByteArrayLowerMemoryAsync {
    instruction: Instruction,
}

impl_async_executable_instruction!(
    byte_array_lower_memory(instruction: Instruction) -> _ {
        Box::new(ByteArrayLowerMemoryAsync{instruction})
    }

    ByteArrayLowerMemoryAsync {
         fn execute<'args>(&'args self, runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) -> BoxFuture<InstructionResult<()>> {
            async move {
                let instruction = &self.instruction;
                let mut inputs = runtime.stack.pop(2).ok_or_else(|| {
                    InstructionError::from_error_kind(
                        instruction.clone(),
                        InstructionErrorKind::StackIsTooSmall { needed: 2 },
                    )
                })?;

                let array_pointer = to_native::<i32>(inputs.remove(0), instruction.clone())? as u32;
                let array: Vec<u8> = to_native(inputs.remove(0), instruction.clone())?;
                let length = array.len() as u32;

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

                memory_view
                    .check_bounds(runtime.store, array_pointer, array.len() as u32)
                    .map_err(|e| InstructionError::from_memory_access(instruction.clone(), e))?;

                memory_view.write_bytes(runtime.store, array_pointer, &array);

                log::debug!("string.lower_memory: pushing {}, {} on the stack", array_pointer, length);
                runtime.stack.push(IValue::I32(array_pointer as i32));
                runtime.stack.push(IValue::I32(length as i32));

                Ok(())
            }.boxed()
        }
    }
);

struct ByteArraySizeAsync {
    instruction: Instruction,
}

impl_async_executable_instruction!(
    byte_array_size(instruction: Instruction) -> _ {
        Box::new(ByteArraySizeAsync{instruction})
    }

    ByteArraySizeAsync {
        fn execute<'args>(&'args self, runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) -> BoxFuture<InstructionResult<()>> {
            async move {
                let instruction = &self.instruction;
                match runtime.stack.pop1() {
                    Some(IValue::ByteArray(array)) => {
                        let length = array.len() as i32;

                        log::debug!("byte_array.size: pushing {} on the stack", length);
                        runtime.stack.push(IValue::I32(length));

                        Ok(())
                    },

                    Some(IValue::Array(array)) => {
                        let array = check_array_type(array, &instruction)?;

                        let length = array.len() as i32;

                        log::debug!("byte_array.size: pushing {} on the stack", length);
                        runtime.stack.push(IValue::I32(length));

                        Ok(())
                    },

                    Some(value) => instr_error!(
                        instruction.clone(),
                        InstructionErrorKind::InvalidValueOnTheStack {
                            expected_type: IType::ByteArray,
                            received_value: (&value).clone(),
                        }
                    ),

                    None => instr_error!(
                        instruction.clone(),
                        InstructionErrorKind::StackIsTooSmall { needed: 1 }
                    ),
                }
            }.boxed()
        }
    }
);

fn check_array_type(
    ivalues: Vec<IValue>,
    instruction: &Instruction,
) -> Result<Vec<IValue>, InstructionError> {
    if ivalues.is_empty() {
        return Ok(ivalues);
    }

    match &ivalues[0] {
        IValue::U8(_) => Ok(ivalues),
        _ => instr_error!(
            instruction.clone(),
            InstructionErrorKind::InvalidValueOnTheStack {
                expected_type: IType::ByteArray,
                received_value: IValue::Array(ivalues),
            }
        ),
    }
}
