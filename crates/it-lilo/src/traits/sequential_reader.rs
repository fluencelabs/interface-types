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
use crate::read_ty_decl;

pub trait SequentialReader {
    fn read_bool(&self) -> bool;

    read_ty_decl!(read_u8, u8, 1);
    read_ty_decl!(read_i8, i8, 1);
    read_ty_decl!(read_u16, u16, 2);
    read_ty_decl!(read_i16, i16, 2);
    read_ty_decl!(read_u32, u32, 4);
    read_ty_decl!(read_i32, i32, 4);
    read_ty_decl!(read_f32, f32, 4);
    read_ty_decl!(read_u64, u64, 8);
    read_ty_decl!(read_i64, i64, 8);
    read_ty_decl!(read_f64, f64, 8);
}