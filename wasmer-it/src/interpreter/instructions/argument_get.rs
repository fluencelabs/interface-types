use crate::instr_error;
use crate::interpreter::instructions::InstructionErrorKind;
use crate::interpreter::stack::Stackable;
use crate::interpreter::Instruction;
use crate::interpreter::InstructionResult;
use crate::interpreter::Runtime;

use futures::future::BoxFuture;
use futures::FutureExt;

struct ArgumentGetAsync {
    index: u32,
    instruction: Instruction,
}

impl_async_executable_instruction!(
    argument_get(index: u32, instruction: Instruction) -> _ {
        Box::new(ArgumentGetAsync {index, instruction})
    }
    ArgumentGetAsync {
        fn execute<'args>(&'args self, runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) ->
        BoxFuture<InstructionResult<()>> {
            async move {
                let invocation_inputs = runtime.invocation_inputs;

                if (self.index as usize) >= invocation_inputs.len() {
                    return instr_error!(
                        self.instruction.clone(),
                        InstructionErrorKind::InvocationInputIsMissing { index: self.index }
                    );
                }

                log::debug!("arg.get: pushing {:?} on the stack", invocation_inputs[self.index as usize]);

                runtime.stack.push(invocation_inputs[self.index as usize].clone());

                Ok(())
            }.boxed()
        }
    }
);

#[cfg(test)]
mod tests {
    test_executable_instruction!(
        test_argument_get =
            instructions: [Instruction::ArgumentGet { index: 0 }],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_argument_get__twice =
            instructions: [
                Instruction::ArgumentGet { index: 0 },
                Instruction::ArgumentGet { index: 1 },
            ],
            invocation_inputs: [
                IValue::I32(7),
                IValue::I32(42),
            ],
            instance: Instance::new(),
            stack: [
                IValue::I32(7),
                IValue::I32(42),
            ],
    );

    test_executable_instruction!(
        test_argument_get__invalid_index =
            instructions: [Instruction::ArgumentGet { index: 1 }],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            error: "`arg.get 1` cannot access invocation inputs #1 because it doesn't exist"
    );
}
