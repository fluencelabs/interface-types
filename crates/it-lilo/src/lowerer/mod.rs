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

mod error;
mod lower_array;
mod lower_record;
mod memory_writer;

use crate::lowerer::memory_writer::MemoryWriter;
use crate::traits::Allocatable;

pub use error::LoError;
pub use lower_array::array_lower_memory;
pub use lower_array::LoweredArray;
pub use lower_record::record_lower_memory;

pub use it_tratis::MemoryView;

pub type LoResult<T> = std::result::Result<T, error::LoError>;

pub struct ILowerer<'m, A: Allocatable, MV: MemoryView> {
    pub writer: MemoryWriter<'m, A, MV>,
}

impl<'m, A: Allocatable, MV: MemoryView> ILowerer<'m, A, MV> {
    pub fn new(view: MV, allocatable: &'m A) -> LoResult<Self> {
        let writer = MemoryWriter::new(view, allocatable)?;
        let lowerer = Self { writer };

        Ok(lowerer)
    }
}
