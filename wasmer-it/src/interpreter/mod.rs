//! A stack-based interpreter to execute instructions of WIT adapters.

mod instructions;
pub mod stack;
pub mod wasm;

pub use instructions::Instruction;

use crate::errors::{InstructionResult, InterpreterResult};
use crate::IValue;
use stack::Stack;
use std::{convert::TryFrom, marker::PhantomData};

/// Represents the `Runtime`, which is used by an adapter to execute
/// its instructions.
pub(crate) struct Runtime<'invocation, 'instance, Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: wasm::structures::Export + 'instance,
    LocalImport: wasm::structures::LocalImport + 'instance,
    Memory: wasm::structures::Memory<MemoryView> + 'instance,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView> + 'instance,
{
    /// The invocation inputs are all the arguments received by an
    /// adapter.
    invocation_inputs: &'invocation [IValue],

    /// Each runtime (so adapter) has its own stack instance.
    stack: Stack<IValue>,

    /// The WebAssembly module instance. It is used by adapter's
    /// instructions.
    wasm_instance: &'instance mut Instance,

    /// Phantom data.
    _phantom: PhantomData<(Export, LocalImport, Memory, MemoryView)>,
}

/// Type alias for an executable instruction. It's an implementation
/// details, but an instruction is a boxed closure instance.
pub(crate) type ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView> = Box<
    dyn Fn(&mut Runtime<Instance, Export, LocalImport, Memory, MemoryView>) -> InstructionResult<()>
        + Send,
>;

/// An interpreter is the central piece of this crate. It is a set of
/// executable instructions. Each instruction takes the runtime as
/// argument. The runtime holds the invocation inputs, [the
/// stack](stack), and [the WebAssembly instance](wasm).
///
/// When the interpreter executes the instructions, each of them can
/// query the WebAssembly instance, operates on the stack, or reads
/// the invocation inputs. At the end of the execution, the stack
/// supposedly contains a result. Since an interpreter is used by a
/// WIT adapter to execute its instructions, the result on the stack
/// is the result of the adapter.
///
/// # Example
///
/// ```rust,ignore
/// use std::{cell::Cell, collections::HashMap, convert::TryInto};
/// use wasmer_interface_types::{
///     interpreter::{
///         instructions::tests::{Export, Instance, LocalImport, Memory, MemoryView},
/// //      ^^^^^^^^^^^^ This is private and for testing purposes only.
/// //                   It is basically a fake WebAssembly runtime.
///         stack::Stackable,
///         Instruction, Interpreter,
///     },
///     types::IType,
///     values::IValue,
/// };
///
/// // 1. Creates an interpreter from a set of instructions. They will
/// //    be transformed into executable instructions.
/// let interpreter: Interpreter<Instance, Export, LocalImport, Memory, MemoryView> = (&vec![
///     Instruction::ArgumentGet { index: 1 },
///     Instruction::ArgumentGet { index: 0 },
///     Instruction::CallCore { function_index: 42 },
/// ])
///     .try_into()
///     .unwrap();
///
/// // 2. Defines the arguments of the adapter.
/// let invocation_inputs = vec![IValue::I32(3), IValue::I32(4)];
///
/// // 3. Creates a WebAssembly instance.
/// let mut instance = Instance {
///     // 3.1. Defines one function: `fn sum(a: i32, b: i32) -> i32 { a + b }`.
///     locals_or_imports: {
///         let mut hashmap = HashMap::new();
///         hashmap.insert(
///             42,
///             LocalImport {
///                 // Defines the argument types of the function.
///                 inputs: vec![IType::I32, IType::I32],
///
///                 // Defines the result types.
///                 outputs: vec![IType::I32],
///
///                 // Defines the function implementation.
///                 function: |arguments: &[IValue]| {
///                     let a: i32 = (&arguments[0]).try_into().unwrap();
///                     let b: i32 = (&arguments[1]).try_into().unwrap();
///
///                     Ok(vec![IValue::I32(a + b)])
///                 },
///             },
///         );
///     },
///     ..Default::default()
/// };
///
/// // 4. Executes the instructions.
/// let run = interpreter.run(&invocation_inputs, &mut instance);
///
/// assert!(run.is_ok());
///
/// let stack = run.unwrap();
///
/// // 5. Read the stack to get the result.
/// assert_eq!(stack.as_slice(), &[IValue::I32(7)]);
/// ```
pub struct Interpreter<Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: wasm::structures::Export,
    LocalImport: wasm::structures::LocalImport,
    Memory: wasm::structures::Memory<MemoryView>,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    executable_instructions:
        Vec<ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView>>,
}

impl<Instance, Export, LocalImport, Memory, MemoryView>
    Interpreter<Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: wasm::structures::Export,
    LocalImport: wasm::structures::LocalImport,
    Memory: wasm::structures::Memory<MemoryView>,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    fn iter(
        &self,
    ) -> impl Iterator<
        Item = &ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView>,
    > + '_ {
        self.executable_instructions.iter()
    }

    /// Runs the interpreter, such as:
    ///   1. Create a fresh stack,
    ///   2. Create a fresh stack,
    ///   3. Execute the instructions one after the other, and
    ///      returns the stack.
    pub fn run(
        &self,
        invocation_inputs: &[IValue],
        wasm_instance: &mut Instance,
    ) -> InterpreterResult<Stack<IValue>> {
        let mut runtime = Runtime {
            invocation_inputs,
            stack: Stack::new(),
            wasm_instance,
            _phantom: PhantomData,
        };

        for executable_instruction in self.iter() {
            executable_instruction(&mut runtime)?;
        }

        Ok(runtime.stack)
    }
}

/// Transforms a `Vec<Instruction>` into an `Interpreter`.
impl<Instance, Export, LocalImport, Memory, MemoryView> TryFrom<Vec<Instruction>>
    for Interpreter<Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: wasm::structures::Export,
    LocalImport: wasm::structures::LocalImport,
    Memory: wasm::structures::Memory<MemoryView>,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    type Error = ();

    fn try_from(instructions: Vec<Instruction>) -> Result<Self, Self::Error> {
        let executable_instructions = instructions
            .into_iter()
            .map(|instruction| match instruction {
                Instruction::ArgumentGet { index } => {
                    instructions::argument_get(index, instruction)
                }

                Instruction::CallCore { function_index } => {
                    instructions::call_core(function_index, instruction)
                }

                Instruction::BoolFromI32 => instructions::bool_from_i32(instruction),
                Instruction::S8FromI32 => instructions::s8_from_i32(instruction),
                Instruction::S8FromI64 => instructions::s8_from_i64(instruction),
                Instruction::S16FromI32 => instructions::s16_from_i32(instruction),
                Instruction::S16FromI64 => instructions::s16_from_i64(instruction),
                Instruction::S32FromI32 => instructions::s32_from_i32(instruction),
                Instruction::S32FromI64 => instructions::s32_from_i64(instruction),
                Instruction::S64FromI32 => instructions::s64_from_i32(instruction),
                Instruction::S64FromI64 => instructions::s64_from_i64(instruction),
                Instruction::I32FromBool => instructions::i32_from_bool(instruction),
                Instruction::I32FromS8 => instructions::i32_from_s8(instruction),
                Instruction::I32FromS16 => instructions::i32_from_s16(instruction),
                Instruction::I32FromS32 => instructions::i32_from_s32(instruction),
                Instruction::I32FromS64 => instructions::i32_from_s64(instruction),
                Instruction::I64FromS8 => instructions::i64_from_s8(instruction),
                Instruction::I64FromS16 => instructions::i64_from_s16(instruction),
                Instruction::I64FromS32 => instructions::i64_from_s32(instruction),
                Instruction::I64FromS64 => instructions::i64_from_s64(instruction),
                Instruction::U8FromI32 => instructions::u8_from_i32(instruction),
                Instruction::U8FromI64 => instructions::u8_from_i64(instruction),
                Instruction::U16FromI32 => instructions::u16_from_i32(instruction),
                Instruction::U16FromI64 => instructions::u16_from_i64(instruction),
                Instruction::U32FromI32 => instructions::u32_from_i32(instruction),
                Instruction::U32FromI64 => instructions::u32_from_i64(instruction),
                Instruction::U64FromI32 => instructions::u64_from_i32(instruction),
                Instruction::U64FromI64 => instructions::u64_from_i64(instruction),
                Instruction::I32FromU8 => instructions::i32_from_u8(instruction),
                Instruction::I32FromU16 => instructions::i32_from_u16(instruction),
                Instruction::I32FromU32 => instructions::i32_from_u32(instruction),
                Instruction::I32FromU64 => instructions::i32_from_u64(instruction),
                Instruction::I64FromU8 => instructions::i64_from_u8(instruction),
                Instruction::I64FromU16 => instructions::i64_from_u16(instruction),
                Instruction::I64FromU32 => instructions::i64_from_u32(instruction),
                Instruction::I64FromU64 => instructions::i64_from_u64(instruction),
                Instruction::PushI32 { value } => instructions::push_i32(value),
                Instruction::PushI64 { value } => instructions::push_i64(value),

                Instruction::StringLiftMemory => instructions::string_lift_memory(instruction),
                Instruction::StringLowerMemory => instructions::string_lower_memory(instruction),
                Instruction::StringSize => instructions::string_size(instruction),

                Instruction::ByteArrayLiftMemory => {
                    instructions::byte_array_lift_memory(instruction)
                }
                Instruction::ByteArrayLowerMemory => {
                    instructions::byte_array_lower_memory(instruction)
                }
                Instruction::ByteArraySize => instructions::byte_array_size(instruction),

                Instruction::ArrayLiftMemory { ref value_type } => {
                    let value_type = value_type.clone();
                    instructions::array_lift_memory(instruction, value_type)
                }
                Instruction::ArrayLowerMemory { ref value_type } => {
                    let value_type = value_type.clone();
                    instructions::array_lower_memory(instruction, value_type)
                }
                Instruction::RecordLiftMemory { record_type_id } => {
                    instructions::record_lift_memory(record_type_id as _, instruction)
                }
                Instruction::RecordLowerMemory { record_type_id } => {
                    instructions::record_lower_memory(record_type_id as _, instruction)
                }
                Instruction::Dup => instructions::dup(instruction),
                Instruction::Swap2 => instructions::swap2(instruction),
            })
            .collect();

        Ok(Interpreter {
            executable_instructions,
        })
    }
}
