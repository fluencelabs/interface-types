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

use super::traits::RecordResolvable;

pub use it_traits::MemoryView;

pub type LiResult<T> = std::result::Result<T, error::LiError>;

pub struct ILifter<'r, R: RecordResolvable, MV: MemoryView> {
    pub reader: MemoryReader<MV>,
    pub resolver: &'r R,
}

impl<'r, R: RecordResolvable, MV: MemoryView> ILifter<'r, R, MV> {
    pub fn new(view: MV, resolver: &'r R) -> Self {
        let reader = MemoryReader::new(view);
        Self { reader, resolver }
    }
}
