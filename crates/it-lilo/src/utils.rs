/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::IRecordType;
use crate::IType;
use crate::IValue;

/// Size of a value in a serialized view.
pub fn ser_type_size(ty: &IType) -> u32 {
    const WASM_POINTER_SIZE: u32 = 4;

    match ty {
        IType::Boolean | IType::S8 | IType::U8 => 1,
        IType::S16 | IType::U16 => 2,
        IType::S32 | IType::U32 | IType::I32 | IType::F32 => 4,
        IType::Record(_) => 4,
        // Vec-like types are passed by pointer and size
        IType::String | IType::ByteArray | IType::Array(_) => 2 * WASM_POINTER_SIZE,
        IType::S64 | IType::U64 | IType::I64 | IType::F64 => 8,
    }
}

/// Size of a value in a serialized view.
pub fn ser_value_size(value: &IValue) -> u32 {
    match value {
        IValue::Boolean(_) | IValue::S8(_) | IValue::U8(_) => 1,
        IValue::S16(_) | IValue::U16(_) => 2,
        IValue::S32(_) | IValue::U32(_) | IValue::F32(_) | IValue::I32(_) => 4,
        IValue::S64(_) | IValue::U64(_) | IValue::F64(_) | IValue::I64(_) => 8,
        IValue::String(_) | IValue::ByteArray(_) | IValue::Array(_) => 2 * 4,
        IValue::Record(_) => 4,
    }
}

/// Returns the record size in bytes.
pub fn record_size(record_type: &IRecordType) -> u32 {
    record_type
        .fields
        .iter()
        .map(|f| ser_type_size(&f.ty))
        .sum()
}

pub fn type_tag_form_itype(itype: &IType) -> u32 {
    const POINTER_CODE: u32 = 3; // u32 in the sdk

    match itype {
        IType::Boolean => 0,          // u8
        IType::U8 => 1,               // u8
        IType::U16 => 2,              // u16
        IType::U32 => 3,              // u32
        IType::U64 => 4,              // u64
        IType::S8 => 5,               // i8
        IType::S16 => 6,              // i16
        IType::S32 | IType::I32 => 7, // i32
        IType::S64 | IType::I64 => 8, // i64
        IType::F32 => 9,              // f32
        IType::F64 => 10,             // f64
        IType::ByteArray | IType::Array(_) | IType::Record(_) | IType::String => POINTER_CODE,
    }
}

pub fn type_tag_form_ivalue(itype: &IValue) -> u32 {
    const POINTER_CODE: u32 = 3; // u32 in the sdk

    match itype {
        IValue::Boolean(_) => 0,              // u8
        IValue::U8(_) => 1,                   // u8
        IValue::U16(_) => 2,                  // u16
        IValue::U32(_) => 3,                  // u32
        IValue::U64(_) => 4,                  // u64
        IValue::S8(_) => 5,                   // i8
        IValue::S16(_) => 6,                  // i16
        IValue::S32(_) | IValue::I32(_) => 7, // i32
        IValue::S64(_) | IValue::I64(_) => 8, // i64
        IValue::F32(_) => 9,                  // f32
        IValue::F64(_) => 10,                 // f64
        IValue::ByteArray(_) | IValue::Array(_) | IValue::Record(_) | IValue::String(_) => {
            POINTER_CODE
        }
    }
}
