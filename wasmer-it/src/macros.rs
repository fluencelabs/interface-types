//! Collection of helpful macros.

/// This macro creates a `Vec1` by checking at compile-time that its
/// invariant holds.
#[macro_export]
macro_rules! ne_vec {
    ($item:expr; 0) => {
        compile_error!("Cannot create an empty `Vec1`, it violates its invariant.")
    };

    () => {
        compile_error!("Cannot create an empty `Vec1`, it violates its invariant.")
    };

    ($item:expr; $length:expr) => {
        {
            fluence_it_types::ne_vec::NEVec::new(vec![$item; $length]).unwrap()
        }
    };

    ($($item:expr),+ $(,)?) => {
        {
            fluence_it_types::ne_vec::NEVec::new(vec![$($item),*]).unwrap()
        }
    };
}

/// This macro runs a parser, extracts the next input and the parser
/// output, and positions the next input on `$input`.
macro_rules! consume {
    (($input:ident, $parser_output:ident) = $parser_expression:expr) => {
        let (next_input, $parser_output) = $parser_expression;
        $input = next_input;
    };

    (($input:ident, mut $parser_output:ident) = $parser_expression:expr) => {
        let (next_input, mut $parser_output) = $parser_expression;
        $input = next_input;
    };
}

/// This macro creates an executable instruction for the interpreter.
///
/// # Example
///
/// The following example creates a `foo` executable instruction.clone(),
/// which takes 2 arguments (`x` and `y`), and does something
/// mysterious by using the `interpreter::Runtime` API.
///
/// ```rust,ignore
/// executable_instruction!(
///     foo(x: u64, y: u64, instruction_name: String) -> _ {
/// //                                                   ^ output type is purposely blank
/// //                      ^^^^^^^^^^^^^^^^ the instruction name, for debugging purposes
/// //              ^ the `y` argument
/// //      ^ the `x` argument
///
///     // an executable instruction is a closure that takes a `Runtime` instance
///     move |runtime| -> _ {
///         // Do something.
///
///         Ok(())
///     }
/// );
/// ```
///
/// Check the existing executable instruction to get more examples.
macro_rules! executable_instruction {
    ($name:ident ( $($argument_name:ident: $argument_type:ty),* ) -> _ $implementation:block ) => {
        pub(crate) fn $name<Instance, Export, LocalImport, Memory, MemoryView, Store>(
            $($argument_name: $argument_type),*
        ) -> crate::interpreter::ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView, Store>
        where
            Export: crate::interpreter::wasm::structures::Export,
            LocalImport: crate::interpreter::wasm::structures::LocalImport<Store>,
            Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
            MemoryView: crate::interpreter::wasm::structures::MemoryView,
            Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView, Store>,
            Store: crate::interpreter::wasm::structures::Store,
        {
            #[allow(unused_imports)]
            use crate::interpreter::{stack::Stackable};

            Box::new($implementation)
        }
    };
}

#[cfg(test)]
macro_rules! test_executable_instruction {
    (
        $test_name:ident =
            instructions: [ $($instructions:expr),* $(,)* ],
            invocation_inputs: [ $($invocation_inputs:expr),* $(,)* ],
            instance: $instance:expr,
            stack: [ $($stack:expr),* $(,)* ]
            $(,)*
    ) => {
        #[test]
        #[allow(non_snake_case, unused)]
        fn $test_name() {
            use crate::{
                interpreter::{
                    instructions::tests::{Export, Instance, LocalImport, Memory, MemoryView},
                    stack::Stackable,
                    Instruction, Interpreter,
                },
                types::IType,
                values::IValue,
            };
            use std::{cell::Cell, collections::HashMap, convert::TryInto};

            let interpreter: Interpreter<Instance, Export, LocalImport, Memory, MemoryView> =
                (&vec![$($instructions),*]).try_into().unwrap();

            let invocation_inputs = vec![$($invocation_inputs),*];
            let mut instance = $instance;
            let run = interpreter.run(&invocation_inputs, &mut instance);

            let err = match &run {
                Ok(_) => "".to_string(),
                Err(e) => e.to_string(),
            };

            assert!(run.is_ok(), err);

            let stack = run.unwrap();

            assert_eq!(stack.as_slice(), &[$($stack),*]);
        }
    };

    (
        $test_name:ident =
            instructions: [ $($instructions:expr),* $(,)* ],
            invocation_inputs: [ $($invocation_inputs:expr),* $(,)* ],
            instance: $instance:expr,
            error: $error:expr
            $(,)*
    ) => {
        #[test]
        #[allow(non_snake_case, unused)]
        fn $test_name() {
            use crate::{
                interpreter::{
                    instructions::tests::{Export, Instance, LocalImport, Memory, MemoryView},
                    stack::Stackable,
                    Instruction, Interpreter,
                },
                types::IType,
                values::IValue,
            };
            use std::{cell::Cell, collections::HashMap, convert::TryInto};

            let interpreter: Interpreter<Instance, Export, LocalImport, Memory, MemoryView> =
                (&vec![$($instructions),*]).try_into().unwrap();

            let invocation_inputs = vec![$($invocation_inputs),*];
            let mut instance = $instance;
            let run = interpreter.run(&invocation_inputs, &mut instance);

            assert!(run.is_err());

            let error = run.unwrap_err().to_string();

            assert_eq!(error, String::from($error));
        }
    };
}
