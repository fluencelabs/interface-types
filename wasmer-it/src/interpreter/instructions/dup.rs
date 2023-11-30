use crate::{
    errors::{InstructionError, InstructionErrorKind, InstructionResult},
    interpreter::stack::Stackable,
    interpreter::Instruction,
    interpreter::Runtime,
};

use futures::future::BoxFuture;
use futures::FutureExt;

struct DupAsync {
    instruction: Instruction,
}

impl_async_executable_instruction!(
    dup(instruction: Instruction) -> _ {
        Box::new(DupAsync{instruction})
    }
    DupAsync {
        fn execute<'args>(&'args self, runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) -> BoxFuture<InstructionResult<()>> {
            async move {
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
            }.boxed()
        }
    }
);
