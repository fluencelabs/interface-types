use crate::{
    errors::{InstructionError, InstructionErrorKind, InstructionResult},
    interpreter::stack::Stackable,
    interpreter::Instruction,
    interpreter::Runtime,
};

struct DupAsync {
    instruction: Instruction,
}

impl_async_executable_instruction!(
    dup(instruction: Instruction) -> _ {
        Box::new(DupAsync{instruction})
    }
    DupAsync {
        async fn execute(&self, runtime: &mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) -> InstructionResult<()> {
            let instruction = &self.instruction;
            let value = runtime.stack.peek1().ok_or_else(|| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            let value = value.clone();
            log::trace!("dup: duplication {:?} value on the stack", value);
            runtime.stack.push(value);

            Ok(())
        }
    }
);
