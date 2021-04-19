//! The error module contains all the data structures that represent
//! an error.

use crate::IType;
use crate::IValue;
use crate::{ast::TypeKind, interpreter::Instruction};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::TryFromIntError,
    result::Result,
    string::{self, ToString},
};

pub use fluence_it_types::WasmValueNativeCastError;

/// A type alias for instruction's results.
pub type InstructionResult<T> = Result<T, InstructionError>;

/// A type alias for the interpreter result.
pub type InterpreterResult<T> = Result<T, InstructionError>;

/// Structure to represent the errors for instructions.
#[derive(Debug)]
pub struct InstructionError {
    /// The instruction that raises the error.
    pub instruction: Instruction,

    /// The error kind.
    pub error_kind: InstructionErrorKind,
}

impl InstructionError {
    pub(crate) fn new(instruction: Instruction, error_kind: InstructionErrorKind) -> Self {
        Self {
            instruction,
            error_kind,
        }
    }
}

impl Error for InstructionError {}

/// Allows you to shorten the expression creates a new InstructionError.
#[macro_export]
macro_rules! instr_error {
    ($instruction:expr, $error_kind:expr) => {
        Err(crate::errors::InstructionError::new(
            $instruction,
            $error_kind,
        ))
    };
}

impl Display for InstructionError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "`{}` {}",
            (&self.instruction).to_string(),
            self.error_kind
        )
    }
}

/// The kind of instruction errors.
#[derive(Debug)]
pub enum InstructionErrorKind {
    /// The instruction needs to read an invocation input at index `index`, but it's missing.
    InvocationInputIsMissing {
        /// The invocation input index.
        index: u32,
    },

    /// Failed to cast from a WIT value to a native value.
    ToNative(WasmValueNativeCastError),

    /// Failed to cast from `from` to `to`.
    LoweringLifting {
        /// The initial type.
        from: IType,

        /// The targeted type.
        to: IType,
    },

    /// Read a value from the stack, but it doesn't have the expected
    /// type.
    InvalidValueOnTheStack {
        /// The expected type.
        expected_type: IType,

        /// The received type.
        received_value: IValue,
    },

    /// Need to read some values from the stack, but it doesn't
    /// contain enough data.
    StackIsTooSmall {
        /// The number of values that were needed.
        needed: usize,
    },

    /// The local or import function doesn't exist.
    LocalOrImportIsMissing {
        /// The local or import function index.
        function_index: u32,
    },

    /// Values given to a local or import function doesn't match the
    /// function signature.
    LocalOrImportSignatureMismatch {
        /// The local or import function index.
        function_index: u32,

        /// The expected signature.
        expected: (Vec<IType>, Vec<IType>),

        /// The received signature.
        received: (Vec<IType>, Vec<IType>),
    },

    /// Failed to call a local or import function.
    LocalOrImportCall {
        /// The local or import function name that has been called.
        function_name: String,
    },

    /// The memory doesn't exist.
    MemoryIsMissing {
        /// The memory index.
        memory_index: usize,
    },

    /// Tried to read out of bounds of the memory.
    MemoryOutOfBoundsAccess {
        /// The access index.
        index: usize,

        /// The memory length.
        length: usize,
    },

    /// The string contains invalid UTF-8 encoding.
    String(string::FromUtf8Error),

    /// Out of range integral type conversion attempted.
    NegativeValue {
        /// The variable name that triggered the error.
        subject: &'static str,
    },

    /// The type doesn't exist.
    TypeIsMissing {
        /// The type index.
        type_index: u32,
    },

    /// The searched by name type doesn't exist.
    RecordTypeByNameIsMissing {
        /// The record type name.
        record_type_id: u64,
    },

    /// Corrupted array's been popped from the stack.
    CorruptedArray(String),

    /// Corrupted record's been popped from the stack.
    CorruptedRecord(String),

    /// Read a type that has an unexpected type.
    InvalidTypeKind {
        /// The expected kind.
        expected_kind: TypeKind,

        /// The received kind.
        received_kind: TypeKind,
    },

    /// Errors related to Serialization/deserialization of record.
    SerdeError(String),

    /// Errors related to lifting incorrect UTF8 string from a Wasm module.
    CorruptedUTF8String(std::string::FromUtf8Error),
}

impl Error for InstructionErrorKind {}

impl Display for InstructionErrorKind {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvocationInputIsMissing { index } => write!(
                formatter,
                "cannot access invocation inputs #{} because it doesn't exist",
                index
            ),

            Self::ToNative(WasmValueNativeCastError { from, .. }) => write!(
                formatter,
                "failed to cast the WIT value `{:?}` to its native type",
                from,
            ),

            Self::LoweringLifting { from, to } => {
                write!(formatter, "failed to cast `{:?}` to `{:?}`", from, to)
            }

            Self::InvalidValueOnTheStack {
                expected_type,
                received_value,
            } => write!(
                formatter,
                "read a value `{:?}` from the stack, that can't be converted to `{:?}`",
                received_value, expected_type,
            ),

            Self::StackIsTooSmall { needed } => write!(
                formatter,
                "needed to read `{}` value(s) from the stack, but it doesn't contain enough data",
                needed
            ),

            Self::LocalOrImportIsMissing { function_index } => write!(
                formatter,
                "the local or import function `{}` doesn't exist",
                function_index
            ),

            Self::LocalOrImportSignatureMismatch { function_index, expected, received } => write!(
                formatter,
                "the local or import function `{}` has the signature `{:?} -> {:?}` but it received values of kind `{:?} -> {:?}`",
                function_index, expected.0, expected.1, received.0, received.1,
            ),

            Self::LocalOrImportCall  { function_name } => write!(
                formatter,
                "failed while calling the local or import function `{}`",
                function_name
            ),

            Self::MemoryIsMissing { memory_index } => write!(
                formatter,
                "memory `{}` does not exist",
                memory_index,
            ),

            Self::MemoryOutOfBoundsAccess { index, length } => write!(
                formatter,
                "read out of the memory bounds (index {} > memory length {})",
                index, length,
            ),

            Self::String(error) => write!(formatter, "{}", error),

            Self::NegativeValue { subject } => write!(
                formatter,
                "attempted to convert `{}` but it appears to be a negative value",
                subject
            ),

            Self::TypeIsMissing { type_index } => write!(
                formatter,
                "the type `{}` doesn't exist",
                type_index
            ),

            Self::InvalidTypeKind { expected_kind, received_kind } => write!(
                formatter,
                "read a type of kind `{:?}`, but the kind `{:?}` was expected",
                received_kind, expected_kind
            ),

            Self::RecordTypeByNameIsMissing  { record_type_id: type_name } => write!(
                formatter,
                "type with `{}` is missing in a Wasm binary",
                type_name
            ),

            Self::CorruptedArray(err) => write!(
                formatter,
                "{}",
                err
            ),

            Self::CorruptedRecord(err) => write!(
                formatter,
                "{}",
                err
            ),

            Self::SerdeError(err) => write!(
                formatter,
                "serde error: {}", err,
            ),

            Self::CorruptedUTF8String(err) => write!(
                formatter,
                "corrupted utf8 string: {}", err
            )
        }
    }
}

impl From<(TryFromIntError, &'static str)> for InstructionErrorKind {
    fn from((_, subject): (TryFromIntError, &'static str)) -> Self {
        InstructionErrorKind::NegativeValue { subject }
    }
}
