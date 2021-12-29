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

use super::LoResult;
use crate::traits::Allocatable;
use crate::traits::DEFAULT_MEMORY_INDEX;
use crate::traits::SequentialWriter;
use crate::utils::type_tag_form_itype;

pub struct MemoryWriter<'i, R: Allocatable> {
    heap_manager: &'i R,
}

impl<'i, A: Allocatable> MemoryWriter<'i, A> {
    pub fn new(heap_manager: &'i A) -> LoResult<Self> {
        let writer = Self {
            heap_manager,
        };
        Ok(writer)
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> LoResult<usize> {
        let byte_type_tag = type_tag_form_itype(&crate::IType::U8);
        let seq_writer = self.sequential_writer(bytes.len() as _, byte_type_tag)?;
        seq_writer.write_bytes(bytes);

        Ok(seq_writer.start_offset())
    }

    pub fn sequential_writer(&self, size: u32, type_tag: u32) -> LoResult<Box<dyn SequentialWriter>> {
        let offset = self.heap_manager.allocate(size, type_tag)?;
        let seq_writer = self.heap_manager.sequential_writer(DEFAULT_MEMORY_INDEX, offset, size as usize)?;
        Ok(seq_writer)
    }
}
