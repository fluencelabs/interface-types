use crate::IValue;

impl_sync_executable_instruction!(
    push_i32(value: i32) -> _ {
        move |runtime| -> _ {

            log::trace!("push_i32: push {} on the stack", value);
            runtime.stack.push(IValue::I32(value));

            Ok(())
        }
    }
);

impl_sync_executable_instruction!(
    push_i64(value: i64) -> _ {
        move |runtime| -> _ {

            log::trace!("push_i32: push {} on the stack", value);
            runtime.stack.push(IValue::I64(value));

            Ok(())
        }
    }
);
