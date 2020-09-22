//! Defines WIT values and associated operations.

use crate::{errors::WasmValueNativeCastError, types::InterfaceType, vec1::Vec1};
use std::{convert::TryFrom, slice::Iter};

#[cfg(feature = "serde")]
pub use crate::serde::{de::from_interface_values, ser::to_interface_value};

/// A WIT value.
#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceValue {
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

    /// A byte array.
    Array(Vec<InterfaceValue>),

    //Anyref(?),
    /// A 32-bits integer (as defined in WebAssembly core).
    I32(i32),

    /// A 64-bits integer (as defiend in WebAssembly core).
    I64(i64),

    /// A record.
    Record(Vec1<InterfaceValue>),
}

impl Default for InterfaceValue {
    fn default() -> Self {
        Self::I32(0)
    }
}

/// Represents a native type supported by WIT.
pub trait NativeType {
    /// The associated interface type that maps to the native type.
    const INTERFACE_TYPE: InterfaceType;
}

macro_rules! native {
    ($native_type:ty, $variant:ident) => {
        impl NativeType for $native_type {
            const INTERFACE_TYPE: InterfaceType = InterfaceType::$variant;
        }

        impl From<$native_type> for InterfaceValue {
            fn from(n: $native_type) -> Self {
                Self::$variant(n)
            }
        }

        impl TryFrom<&InterfaceValue> for $native_type {
            type Error = WasmValueNativeCastError;

            fn try_from(w: &InterfaceValue) -> Result<Self, Self::Error> {
                match w {
                    InterfaceValue::$variant(n) => Ok(n.clone()),
                    _ => Err(WasmValueNativeCastError {
                        from: w.clone(),
                        to: <$native_type>::INTERFACE_TYPE,
                    }),
                }
            }
        }
    };
}

native!(i8, S8);
native!(i16, S16);
native!(i32, I32);
native!(i64, I64);
native!(u8, U8);
native!(u16, U16);
native!(u32, U32);
native!(u64, U64);
native!(f32, F32);
native!(f64, F64);
native!(String, String);

/// Iterates over a vector of `InterfaceValues` but flatten all the
/// values. So `I32(1), Record([I32(2), I32(3)]), I32(4)` will be
/// iterated like `I32(1), I32(2), I32(3), I32(4)`.
pub(crate) struct FlattenInterfaceValueIterator<'a> {
    iterators: Vec<Iter<'a, InterfaceValue>>,
}

impl<'a> FlattenInterfaceValueIterator<'a> {
    pub(crate) fn new(values: &'a [InterfaceValue]) -> Self {
        Self {
            iterators: vec![values.iter()],
        }
    }
}

impl<'a> Iterator for FlattenInterfaceValueIterator<'a> {
    type Item = &'a InterfaceValue;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterators.last_mut()?.next() {
            // End of the current iterator, go back to the previous
            // one.
            None => {
                self.iterators.pop();
                self.next()
            }

            // Recursively iterate over the record.
            Some(InterfaceValue::Record(fields)) => {
                self.iterators.push(fields.iter());
                self.next()
            }

            // A regular item.
            e @ Some(_) => e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! value_to_type {
        ($test_name:ident, $ty:ident, $value:expr) => {
            #[test]
            #[allow(non_snake_case)]
            fn $test_name() {
                assert_eq!(
                    InterfaceType::from(&InterfaceValue::$ty($value)),
                    InterfaceType::$ty
                );
            }
        };
    }

    value_to_type!(interface_type_from_interface_value__s8, S8, 42);
    value_to_type!(interface_type_from_interface_value__s16, S16, 42);
    value_to_type!(interface_type_from_interface_value__s32, S32, 42);
    value_to_type!(interface_type_from_interface_value__s64, S64, 42);
    value_to_type!(interface_type_from_interface_value__u8, U8, 42);
    value_to_type!(interface_type_from_interface_value__u16, U16, 42);
    value_to_type!(interface_type_from_interface_value__u32, U32, 42);
    value_to_type!(interface_type_from_interface_value__u64, U64, 42);
    value_to_type!(interface_type_from_interface_value__f32, F32, 42.);
    value_to_type!(interface_type_from_interface_value__f64, F64, 42.);
    value_to_type!(
        interface_type_from_interface_value__string,
        String,
        "foo".to_string()
    );
    value_to_type!(interface_type_from_interface_value__i32, I32, 42);
    value_to_type!(interface_type_from_interface_value__i64, I64, 42);

    #[test]
    #[allow(non_snake_case)]
    fn interface_type_from_interface_value__record() {
        assert_eq!(
            InterfaceType::from(&InterfaceValue::Record(vec1![
                InterfaceValue::I32(1),
                InterfaceValue::S8(2)
            ])),
            InterfaceType::Record(RecordType {
                fields: vec1![InterfaceType::I32, InterfaceType::S8]
            })
        );

        assert_eq!(
            InterfaceType::from(&InterfaceValue::Record(vec1![
                InterfaceValue::I32(1),
                InterfaceValue::Record(vec1![
                    InterfaceValue::String("a".to_string()),
                    InterfaceValue::F64(42.)
                ]),
                InterfaceValue::S8(2)
            ])),
            InterfaceType::Record(RecordType {
                fields: vec1![
                    InterfaceType::I32,
                    InterfaceType::Record(RecordType {
                        fields: vec1![InterfaceType::String, InterfaceType::F64]
                    }),
                    InterfaceType::S8
                ]
            })
        );
    }
}
