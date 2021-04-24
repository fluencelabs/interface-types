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
mod lift_array;
mod lift_record;
mod macros;
mod memory_reader;

pub use error::LiError;
pub use lift_array::array_lift_memory;
pub use lift_record::record_lift_memory;
pub use memory_reader::MemoryReader;
pub use memory_reader::SequentialReader;

use super::traits::RecordResolvable;

use std::cell::Cell;

pub type LiResult<T> = std::result::Result<T, error::LiError>;

pub struct ILifter<'m, 'r, R: RecordResolvable> {
    pub reader: MemoryReader<'m>,
    pub resolver: &'r R,
}

impl<'m, 'r, R: RecordResolvable> ILifter<'m, 'r, R> {
    pub fn new(memory: &'m [Cell<u8>], resolver: &'r R) -> Self {
        let reader = MemoryReader::new(memory);
        Self { reader, resolver }
    }
}
