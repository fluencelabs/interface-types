use crate::{
    errors::{InstructionError, InstructionErrorKind, InstructionResult},
    interpreter::stack::Stackable,
    interpreter::Instruction,
    interpreter::Runtime,
};

use futures::future::BoxFuture;
use futures::FutureExt;

struct Swap2 {
    instruction: Instruction,
}

impl_async_executable_instruction!(
    swap2(instruction: Instruction) -> _ {
        Box::new(Swap2{instruction})
    }
    Swap2 {
        fn execute<'args>(&'args self, runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) -> BoxFuture<InstructionResult<()>> {
            async move {
                let instruction = &self.instruction;
                let mut values = runtime.stack.pop(2).ok_or_else(|| {
                    InstructionError::from_error_kind(
                        instruction.clone(),
                        InstructionErrorKind::StackIsTooSmall { needed: 1 },
                    )
                })?;

                log::trace!("swap2: swapping {:?}, {:?} values on the stack", values[0], values[1]);
                runtime.stack.push(values.remove(1));
                runtime.stack.push(values.remove(0));

                Ok(())
            }.boxed()
        }
    }
);
