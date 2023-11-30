use crate::IValue;

use crate::interpreter::stack::Stackable;
use crate::interpreter::InstructionResult;
use crate::interpreter::Runtime;

use futures::future::BoxFuture;
use futures::FutureExt;

struct PushI32Async {
    value: i32,
}

impl_async_executable_instruction!(
    push_i32(value: i32) -> _ {
        Box::new(PushI32Async {value})
    }

    PushI32Async {
        fn execute<'args>(&'args self, runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) -> BoxFuture<InstructionResult<()>> {
            async move {
                log::trace!("push_i32: push {} on the stack", self.value);
                runtime.stack.push(IValue::I32(self.value));

                Ok(())
            }.boxed()
        }
    }
);

struct PushI64Async {
    value: i64,
}

impl_async_executable_instruction!(
    push_i64(value: i64) -> _ {
        Box::new(PushI64Async {value})
    }

    PushI64Async {
        fn execute<'args>(&'args self, runtime: &'args mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) -> BoxFuture<InstructionResult<()>> {
            async move {
                log::trace!("push_i32: push {} on the stack", self.value);

                runtime.stack.push(IValue::I64(self.value));

                Ok(())
            }.boxed()
        }
    }
);
