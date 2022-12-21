use crate::instr_error;
use crate::IType;
use crate::IValue;
use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
};

use std::convert::TryInto;

macro_rules! lowering_lifting {
    ($instruction_function_name:ident, $instruction_name:expr, $to_variant:ident, $from_variant:ident) => {
        executable_instruction!(
            $instruction_function_name(instruction: Instruction) -> _ {
                move |runtime, _| -> _ {
                    match runtime.stack.pop1() {
                        Some(IValue::$from_variant(value)) => {
                            runtime
                                .stack
                                .push({
                                    let converted_value = IValue::$to_variant(value.try_into().map_err(
                                        |_| {
                                            InstructionError::from_error_kind(
                                                instruction.clone(),
                                                InstructionErrorKind::LoweringLifting {
                                                    from: IType::$from_variant,
                                                    to: IType::$to_variant
                                                },
                                            )
                                       },
                                    )?);

                                    log::trace!("{}: converting {:?} to {:?}", $instruction_name, value, converted_value);

                                    converted_value
                                })
                        }
                        Some(wrong_value) => {
                            return instr_error!(
                                instruction.clone(),
                                InstructionErrorKind::InvalidValueOnTheStack {
                                    expected_type: IType::$from_variant,
                                    received_value: wrong_value,
                                }
                            )
                        },

                        None => {
                            return instr_error!(
                                instruction.clone(),
                                InstructionErrorKind::StackIsTooSmall { needed: 1 }
                            )
                        }
                    }

                    Ok(())
                }
            }
        );
    };
}

lowering_lifting!(s8_from_i32, "s8.from_i32", S8, I32);
lowering_lifting!(s8_from_i64, "s8.from_i64", S8, I64);
lowering_lifting!(s16_from_i32, "s16.from_i32", S16, I32);
lowering_lifting!(s16_from_i64, "s16.from_i64", S16, I64);
lowering_lifting!(s32_from_i32, "s32.from_i32", S32, I32);
lowering_lifting!(s32_from_i64, "s32.from_i64", S32, I64);
lowering_lifting!(s64_from_i32, "s64.from_i32", S64, I32);
lowering_lifting!(s64_from_i64, "s64.from_i64", S64, I64);
lowering_lifting!(i32_from_bool, "i32.from_bool", I32, Boolean);
lowering_lifting!(i32_from_s8, "i32.from_s8", I32, S8);
lowering_lifting!(i32_from_s16, "i32.from_s16", I32, S16);
lowering_lifting!(i32_from_s32, "i32.from_s32", I32, S32);
lowering_lifting!(i32_from_s64, "i32.from_s64", I32, S64);
lowering_lifting!(i64_from_s8, "i64.from_s8", I64, S8);
lowering_lifting!(i64_from_s16, "i64.from_s16", I64, S16);
lowering_lifting!(i64_from_s32, "i64.from_s32", I64, S32);
lowering_lifting!(i64_from_s64, "i64.from_s64", I64, S64);
lowering_lifting!(u8_from_i32, "u8.from_i32", U8, I32);
lowering_lifting!(u8_from_i64, "u8.from_i64", U8, I64);
lowering_lifting!(u16_from_i32, "u16.from_i32", U16, I32);
lowering_lifting!(u16_from_i64, "u16.from_i64", U16, I64);
lowering_lifting!(u32_from_i32, "u32.from_i32", U32, I32);
lowering_lifting!(u32_from_i64, "u32.from_i64", U32, I64);
lowering_lifting!(u64_from_i32, "u64.from_i32", U64, I32);
lowering_lifting!(u64_from_i64, "u64.from_i64", U64, I64);
lowering_lifting!(i32_from_u8, "i32.from_u8", I32, U8);
lowering_lifting!(i32_from_u16, "i32.from_u16", I32, U16);
lowering_lifting!(i32_from_u32, "i32.from_u32", I32, U32);
lowering_lifting!(i32_from_u64, "i32.from_u64", I32, U64);
lowering_lifting!(i64_from_u8, "i64.from_u8", I64, U8);
lowering_lifting!(i64_from_u16, "i64.from_u16", I64, U16);
lowering_lifting!(i64_from_u32, "i64.from_u32", I64, U32);
lowering_lifting!(i64_from_u64, "i64.from_u64", I64, U64);

executable_instruction!(
    bool_from_i32(instruction: Instruction) -> _ {
        move |runtime, _| -> _ {
            match runtime.stack.pop1() {
                Some(IValue::I32(value)) => {
                    runtime
                        .stack
                        .push({
                            let converted_value = IValue::Boolean(value != 0);

                            log::trace!("bool.from_i32: converting {:?} to {:?}" , value, converted_value);

                            converted_value
                        })
                }
                Some(wrong_value) => {
                    return instr_error!(
                        instruction.clone(),
                        InstructionErrorKind::InvalidValueOnTheStack {
                            expected_type: IType::I32,
                            received_value: wrong_value,
                        }
                    )
                },

                None => {
                    return instr_error!(
                        instruction.clone(),
                        InstructionErrorKind::StackIsTooSmall { needed: 1 }
                    )
                }
            }

            Ok(())
        }
    }
);

#[cfg(test)]
mod tests {
    test_executable_instruction!(
        test_convert_fails =
            instructions: [Instruction::ArgumentGet { index: 0}, Instruction::S8FromI32],
            invocation_inputs: [IValue::I32(128)],
            instance: Instance::new(),
            error: "`s8.from_i32` failed to cast `I32` to `S8`"
    );

    test_executable_instruction!(
        test_type_mismatch =
            instructions: [Instruction::ArgumentGet { index: 0}, Instruction::S8FromI32],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            error: "`s8.from_i32` read a value of type `I64` from the stack, but the type `I32` was expected"
    );

    test_executable_instruction!(
        test_no_value_on_the_stack =
            instructions: [Instruction::S8FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            error: "`s8.from_i32` needed to read `1` value(s) from the stack, but it doesn't contain enough data"
    );

    test_executable_instruction!(
        test_s8_from_i32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::S8FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::S8(42)],
    );

    test_executable_instruction!(
        test_s8_from_i64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::S8FromI64],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            stack: [IValue::S8(42)],
    );

    test_executable_instruction!(
        test_s16_from_i32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::S16FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::S16(42)],
    );

    test_executable_instruction!(
        test_s16_from_i64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::S16FromI64],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            stack: [IValue::S16(42)],
    );

    test_executable_instruction!(
        test_s32_from_i32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::S32FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::S32(42)],
    );

    test_executable_instruction!(
        test_s32_from_i64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::S32FromI64],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            stack: [IValue::S32(42)],
    );

    test_executable_instruction!(
        test_s64_from_i32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::S64FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::S64(42)],
    );

    test_executable_instruction!(
        test_s64_from_i64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::S64FromI64],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            stack: [IValue::S64(42)],
    );

    test_executable_instruction!(
        test_i32_from_s8 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I32FromS8],
            invocation_inputs: [IValue::S8(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_i32_from_s16 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I32FromS16],
            invocation_inputs: [IValue::S16(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_i32_from_s32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I32FromS32],
            invocation_inputs: [IValue::S32(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_i32_from_s64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I32FromS64],
            invocation_inputs: [IValue::S64(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_i64_from_s8 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I64FromS8],
            invocation_inputs: [IValue::S8(42)],
            instance: Instance::new(),
            stack: [IValue::I64(42)],
    );

    test_executable_instruction!(
        test_i64_from_s16 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I64FromS16],
            invocation_inputs: [IValue::S16(42)],
            instance: Instance::new(),
            stack: [IValue::I64(42)],
    );

    test_executable_instruction!(
        test_i64_from_s32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I64FromS32],
            invocation_inputs: [IValue::S32(42)],
            instance: Instance::new(),
            stack: [IValue::I64(42)],
    );

    test_executable_instruction!(
        test_i64_from_s64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I64FromS64],
            invocation_inputs: [IValue::S64(42)],
            instance: Instance::new(),
            stack: [IValue::I64(42)],
    );

    test_executable_instruction!(
        test_u8_from_i32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::U8FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::U8(42)],
    );

    test_executable_instruction!(
        test_u8_from_i64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::U8FromI64],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            stack: [IValue::U8(42)],
    );

    test_executable_instruction!(
        test_u16_from_i32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::U16FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::U16(42)],
    );

    test_executable_instruction!(
        test_u16_from_i64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::U16FromI64],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            stack: [IValue::U16(42)],
    );

    test_executable_instruction!(
        test_u32_from_i32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::U32FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::U32(42)],
    );

    test_executable_instruction!(
        test_u32_from_i64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::U32FromI64],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            stack: [IValue::U32(42)],
    );

    test_executable_instruction!(
        test_u64_from_i32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::U64FromI32],
            invocation_inputs: [IValue::I32(42)],
            instance: Instance::new(),
            stack: [IValue::U64(42)],
    );

    test_executable_instruction!(
        test_u64_from_i64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::U64FromI64],
            invocation_inputs: [IValue::I64(42)],
            instance: Instance::new(),
            stack: [IValue::U64(42)],
    );

    test_executable_instruction!(
        test_i32_from_u8 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I32FromU8],
            invocation_inputs: [IValue::U8(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_i32_from_u16 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I32FromU16],
            invocation_inputs: [IValue::U16(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_i32_from_u32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I32FromU32],
            invocation_inputs: [IValue::U32(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_i32_from_u64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I32FromU64],
            invocation_inputs: [IValue::U64(42)],
            instance: Instance::new(),
            stack: [IValue::I32(42)],
    );

    test_executable_instruction!(
        test_i64_from_u8 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I64FromU8],
            invocation_inputs: [IValue::U8(42)],
            instance: Instance::new(),
            stack: [IValue::I64(42)],
    );

    test_executable_instruction!(
        test_i64_from_u16 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I64FromU16],
            invocation_inputs: [IValue::U16(42)],
            instance: Instance::new(),
            stack: [IValue::I64(42)],
    );

    test_executable_instruction!(
        test_i64_from_u32 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I64FromU32],
            invocation_inputs: [IValue::U32(42)],
            instance: Instance::new(),
            stack: [IValue::I64(42)],
    );

    test_executable_instruction!(
        test_i64_from_u64 =
            instructions: [Instruction::ArgumentGet { index: 0 }, Instruction::I64FromU64],
            invocation_inputs: [IValue::U64(42)],
            instance: Instance::new(),
            stack: [IValue::I64(42)],
    );
}
