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

use it_lilo_utils::error::MemoryWriteError;
use thiserror::Error as ThisError;

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
    pub(crate) fn from_error_kind(
        instruction: Instruction,
        error_kind: InstructionErrorKind,
    ) -> Self {
        Self {
            instruction,
            error_kind,
        }
    }

    pub(crate) fn from_lilo(instruction: Instruction, lilo: LiLoError) -> Self {
        let error_kind = InstructionErrorKind::LiLoError(lilo);
        Self::from_error_kind(instruction, error_kind)
    }

    pub(crate) fn from_write_error(
        instruction: Instruction,
        write_error: MemoryWriteError,
    ) -> Self {
        let error_kind = InstructionErrorKind::MemoryWriteError(write_error);
        Self::from_error_kind(instruction, error_kind)
    }
}

impl Error for InstructionError {}

/// Allows you to shorten the expression creates a new InstructionError.
#[macro_export]
macro_rules! instr_error {
    ($instruction:expr, $error_kind:expr) => {
        Err(crate::errors::InstructionError::from_error_kind(
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
#[derive(ThisError, Debug)]
pub enum InstructionErrorKind {
    /// The instruction needs to read an invocation input at index `index`, but it's missing.
    #[error("cannot access invocation inputs #{index} because it doesn't exist")]
    InvocationInputIsMissing {
        /// The invocation input index.
        index: u32,
    },

    /// Failed to cast from a WIT value to a native value.
    #[error("failed to cast the WIT value `{0}` to its native type")]
    ToNative(#[from] WasmValueNativeCastError),

    /// Failed to cast from `from` to `to`.
    #[error("failed to cast `{from:?}` to `{to:?}`")]
    LoweringLifting {
        /// The initial type.
        from: IType,

        /// The targeted type.
        to: IType,
    },

    /// Read a value from the stack, but it doesn't have the expected
    /// type.
    #[error("read a value `{expected_type:?}` from the stack, that can't be converted to `{received_value:?}`")]
    InvalidValueOnTheStack {
        /// The expected type.
        expected_type: IType,

        /// The received type.
        received_value: IValue,
    },

    /// Need to read some values from the stack, but it doesn't
    /// contain enough data.
    #[error(
        "needed to read `{needed}` value(s) from the stack, but it doesn't contain enough data"
    )]
    StackIsTooSmall {
        /// The number of values that were needed.
        needed: usize,
    },

    /// The local or import function doesn't exist.
    #[error("the local or import function `{function_index}` doesn't exist")]
    LocalOrImportIsMissing {
        /// The local or import function index.
        function_index: u32,
    },

    /// Values given to a local or import function doesn't match the
    /// function signature.
    #[error(
        "the local or import function `{function_index}` has the signature\
             `{:?} -> {:?}`\
             but it received values of kind `{:?} -> {:?}`",
        .expected.0, .expected.1, .received.0, .received.1,
    )]
    LocalOrImportSignatureMismatch {
        /// The local or import function index.
        function_index: u32,

        /// The expected signature.
        expected: (Vec<IType>, Vec<IType>),

        /// The received signature.
        received: (Vec<IType>, Vec<IType>),
    },

    /// Failed to call a local or import function.
    #[error("failed while calling the local or import function `{function_name}`")]
    LocalOrImportCall {
        /// The local or import function name that has been called.
        function_name: String,
    },

    /// The memory doesn't exist.
    #[error("memory `{memory_index}` does not exist")]
    MemoryIsMissing {
        /// The memory index.
        memory_index: usize,
    },

    /// Tried to read out of bounds of the memory.
    #[error("read out of the memory bounds (index {index} > memory length {length})")]
    MemoryOutOfBoundsAccess {
        /// The access index.
        index: usize,

        /// The memory length.
        length: usize,
    },

    /// The string contains invalid UTF-8 encoding.
    #[error("{0}")]
    String(string::FromUtf8Error),

    /// Out of range integral type conversion attempted.
    #[error("attempted to convert `{subject}`, but it appears to be a negative value")]
    NegativeValue {
        /// The variable name that triggered the error.
        subject: &'static str,
    },

    /// The type doesn't exist.
    #[error("the type `{type_index}` doesn't exist")]
    TypeIsMissing {
        /// The type index.
        type_index: u32,
    },

    /// The searched by id type doesn't exist.
    #[error("type with `{record_type_id}` is missing in a Wasm binary")]
    RecordTypeByNameIsMissing {
        /// The record type name.
        record_type_id: u64,
    },

    /// Corrupted array's been popped from the stack.
    #[error("{0}")]
    CorruptedArray(String),

    /// Corrupted record's been popped from the stack.
    #[error("{0}")]
    CorruptedRecord(String),

    /// Read a type that has an unexpected type.
    #[error(
        "read a type of kind `{received_kind:?}`,\
             but the kind `{expected_kind:?}` was expected"
    )]
    InvalidTypeKind {
        /// The expected kind.
        expected_kind: TypeKind,

        /// The received kind.
        received_kind: TypeKind,
    },

    /// Errors related to Serialization/deserialization of record.
    #[error("serde error: {0}")]
    SerdeError(String),

    /// Errors related to lifting/lowering records.
    #[error("{0}")]
    LiLoError(#[from] LiLoError),

    /// Errors related to incorrect writing to memory.
    #[error("{0}")]
    MemoryWriteError(#[from] MemoryWriteError),
}

impl From<(TryFromIntError, &'static str)> for InstructionErrorKind {
    fn from((_, subject): (TryFromIntError, &'static str)) -> Self {
        InstructionErrorKind::NegativeValue { subject }
    }
}

/// Contains various errors encountered while lifting/lowering records and arrays.
#[derive(Debug, ThisError)]
pub enum LiLoError {
    /// This error occurred from out-of-bound memory access.
    #[error("{0}")]
    MemoryAccessError(#[from] it_lilo_utils::error::MemoryAccessError),

    /// An error related to not found record in module record types.
    #[error("Record with type id {0} not found")]
    RecordTypeNotFound(u64),

    /// The memory doesn't exist.
    #[error("memory `{memory_index}` does not exist")]
    MemoryIsMissing {
        /// The memory index.
        memory_index: usize,
    },

    /// The local or import function doesn't exist.
    #[error("the allocate function with index `{function_index}` doesn't exist in Wasm module")]
    AllocateFuncIsMissing {
        /// The local or import function index.
        function_index: u32,
    },

    /// Failed to call a allocate function.
    #[error("call to allocated was failed")]
    AllocateCallFailed,

    /// Allocate input types doesn't match with needed.
    #[error(
        "allocate func doesn't receive two i32 values,\
             probably a Wasm module's built with unsupported sdk version"
    )]
    AllocateFuncIncompatibleSignature,

    /// Allocate output types doesn't match with needed.
    #[error(
        "allocate func doesn't return a one value of I32 type,\
             probably a Wasm module's built with unsupported sdk version"
    )]
    AllocateFuncIncompatibleOutput,

    /// The searched by id type doesn't exist.
    #[error("type with `{record_type_id}` is missing in a Wasm binary")]
    RecordTypeByNameIsMissing {
        /// The record type name.
        record_type_id: u64,
    },

    /// Errors related to lifting incorrect UTF8 string from a Wasm module.
    #[error("corrupted UTF8 string {0}")]
    CorruptedUTF8String(#[from] std::string::FromUtf8Error),

    /// This error occurred when a record is created from empty values array.
    #[error("Record with name '{0}' can't be empty")]
    EmptyRecord(String),
}
