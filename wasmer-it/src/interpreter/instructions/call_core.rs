use crate::{
    errors::{InstructionError, InstructionErrorKind, InstructionResult},
    interpreter::stack::Stackable,
    interpreter::wasm::structures::{FunctionIndex, TypedIndex},
    interpreter::Instruction,
    interpreter::Runtime,
};

struct CallCoreAsync {
    function_index: u32,
    instruction: Instruction,
}

impl_async_executable_instruction!(
    call_core(function_index: u32, instruction: Instruction) -> _ {
        Box::new(CallCoreAsync{function_index, instruction})
    }

    CallCoreAsync {
        async fn execute(&self, runtime: &mut Runtime<Instance, Export, LocalImport, Memory, MemoryView, Store>) -> InstructionResult<()> {
            let instruction = &self.instruction;
            let function_index = self.function_index;

            let instance = &runtime.wasm_instance;
            let index = FunctionIndex::new(function_index as usize);

            let local_or_import = instance.local_or_import(index).ok_or_else(|| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::LocalOrImportIsMissing {
                        function_index,
                    },
                )
            })?;
            let inputs_cardinality = local_or_import.inputs_cardinality();

            let inputs = runtime.stack.pop(inputs_cardinality).ok_or_else(|| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall {
                        needed: inputs_cardinality,
                    },
                )
            })?;

            super::check_function_signature(&**instance, local_or_import, &inputs)
                .map_err(|e| InstructionError::from_error_kind(instruction.clone(), e))?;

            log::debug!("call-core: calling {} with arguments: {:?}", local_or_import.name(), inputs);

            let outputs = local_or_import.call_async(runtime.store, &inputs).await.map_err(|e| {
                InstructionError::from_error_kind(
                    instruction.clone(),
                    InstructionErrorKind::LocalOrImportCall {
                        function_name: local_or_import.name().to_string(),
                        reason: e
                    },
                )
            })?;

            log::debug!("call-core: call to {} succeeded with result {:?}", local_or_import.name(), outputs);

            for output in outputs.into_iter() {
                runtime.stack.push(output)
            }

            Ok(())
        }
    }
);

#[cfg(test)]
mod tests {
    test_executable_instruction!(
        test_call_core =
            instructions: [
                Instruction::ArgumentGet { index: 0 },
                Instruction::ArgumentGet { index: 1 },
                Instruction::CallCore { function_index: 42 },
            ],
            invocation_inputs: [
                IValue::I32(3),
                IValue::I32(4),
            ],
            instance: Instance::new(),
            stack: [IValue::I32(12)],
    );

    test_executable_instruction!(
        test_call_core__invalid_local_import_index =
            instructions: [
                Instruction::CallCore { function_index: 42 },
            ],
            invocation_inputs: [
                IValue::I32(3),
                IValue::I32(4),
            ],
            instance: Default::default(),
            error: r#"`call-core 42` the local or import function `42` doesn't exist"#,
    );

    test_executable_instruction!(
        test_call_core__stack_is_too_small =
            instructions: [
                Instruction::ArgumentGet { index: 0 },
                Instruction::CallCore { function_index: 42 },
                //                                      ^^ `42` expects 2 values on the stack, only one is present
            ],
            invocation_inputs: [
                IValue::I32(3),
                IValue::I32(4),
            ],
            instance: Instance::new(),
            error: r#"`call-core 42` needed to read `2` value(s) from the stack, but it doesn't contain enough data"#,
    );

    test_executable_instruction!(
        test_call_core__invalid_types_in_the_stack =
            instructions: [
                Instruction::ArgumentGet { index: 0 },
                Instruction::ArgumentGet { index: 1 },
                Instruction::CallCore { function_index: 42 },
            ],
            invocation_inputs: [
                IValue::I32(3),
                IValue::I64(4),
                //              ^^^ mismatch with `42` signature
            ],
            instance: Instance::new(),
            error: r#"`call-core 42` the local or import function `42` has the signature `[I32, I32] -> []` but it received values of kind `[I32, I64] -> []`"#,
    );

    test_executable_instruction!(
        test_call_core__failure_when_calling =
            instructions: [
                Instruction::ArgumentGet { index: 0 },
                Instruction::ArgumentGet { index: 1 },
                Instruction::CallCore { function_index: 42 },
            ],
            invocation_inputs: [
                IValue::I32(3),
                IValue::I32(4),
            ],
            instance: Instance {
                locals_or_imports: {
                    let mut hashmap = HashMap::new();
                    hashmap.insert(
                        42,
                        LocalImport {
                            inputs: vec![IType::I32, IType::I32],
                            outputs: vec![IType::I32],
                            function: |_| Err(()),
                            //            ^^^^^^^ function fails
                        },
                    );

                    hashmap
                },
                ..Default::default()
            },
            error: r#"`call-core 42` failed while calling the local or import function `42`"#,
    );

    test_executable_instruction!(
        test_call_core__void =
            instructions: [
                Instruction::ArgumentGet { index: 0 },
                Instruction::ArgumentGet { index: 1 },
                Instruction::CallCore { function_index: 42 },
            ],
            invocation_inputs: [
                IValue::I32(3),
                IValue::I32(4),
            ],
            instance: Instance {
                locals_or_imports: {
                    let mut hashmap = HashMap::new();
                    hashmap.insert(
                        42,
                        LocalImport {
                            inputs: vec![IType::I32, IType::I32],
                            outputs: vec![IType::I32],
                            function: |_| Ok(vec![]),
                            //            ^^^^^^^^^^ void
                        },
                    );

                    hashmap
                },
                ..Default::default()
            },
            stack: [],
    );
}
