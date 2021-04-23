//! Defines WIT values and associated operations.

use crate::ne_vec::NEVec;

/// A WIT value.
#[derive(Debug, Clone, PartialEq)]
pub enum IValue {
    /// Boolean value.
    Boolean(bool),

    /// A 8-bits signed integer.
    S8(i8),

    /// A 16-bits signed integer.
    S16(i16),

    /// A 32-bits signed integer.
    S32(i32),

    /// A 64-bits signed integer.
    S64(i64),

    /// A 8-bits unsigned integer.
    U8(u8),

    /// A 16-bits unsigned integer.
    U16(u16),

    /// A 32-bits unsigned integer.
    U32(u32),

    /// A 64-bits unsigned integer.
    U64(u64),

    /// A 32-bits float.
    F32(f32),

    /// A 64-bits float.
    F64(f64),
    
    /// A string.
    String(String),

    /// Specialization of array type for byte vector.
    ByteArray(Vec<u8>),

    /// A byte array.
    Array(Vec<IValue>),

    /// A 32-bits integer (as defined in WebAssembly core).
    I32(i32),

    /// A 64-bits integer (as defined in WebAssembly core).
    I64(i64),

    /// A record.
    Record(NEVec<IValue>),
}

impl Default for IValue {
    fn default() -> Self {
        Self::I32(0)
    }
}
