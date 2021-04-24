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
use crate::traits::MemSlice;
use crate::traits::DEFAULT_MEMORY_INDEX;
use crate::utils::type_tag_form_itype;

use std::cell::Cell;
use crate::lowerer::LoError;

pub struct MemoryWriter<'i, R: Allocatable> {
    heap_manager: &'i R,
    pub(self) memory: Cell<MemSlice<'i>>,
}

pub struct SequentialWriter {
    start_offset: usize,
    offset: Cell<usize>,
}

impl<'i, A: Allocatable> MemoryWriter<'i, A> {
    pub fn new(heap_manager: &'i A) -> LoResult<Self> {
        let mem_slice = heap_manager.memory_slice(DEFAULT_MEMORY_INDEX)?;
        let memory = Cell::new(mem_slice);

        let writer = Self {
            heap_manager,
            memory,
        };
        Ok(writer)
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> LoResult<usize> {
        let byte_type_tag = type_tag_form_itype(&crate::IType::U8);
        let seq_writer = self.sequential_writer(bytes.len() as _, byte_type_tag)?;
        seq_writer.write_bytes(self, bytes);

        Ok(seq_writer.start_offset())
    }

    pub fn sequential_writer(&self, size: u32, type_tag: u32) -> LoResult<SequentialWriter> {
        let offset = self.heap_manager.allocate(size, type_tag)?;
        if offset == 0 {
            return Err(LoError::AllocateWasInvalid);
        }

        let new_mem_slice = self.heap_manager.memory_slice(DEFAULT_MEMORY_INDEX)?;
        self.memory.set(new_mem_slice);

        Ok(SequentialWriter::new(offset))
    }
}

impl SequentialWriter {
    pub(self) fn new(offset: usize) -> Self {
        Self {
            offset: Cell::new(offset),
            start_offset: offset,
        }
    }

    pub fn start_offset(&self) -> usize {
        self.start_offset
    }

    pub fn write_array<A: Allocatable, const N: usize>(
        &self,
        writer: &MemoryWriter<'_, A>,
        values: [u8; N],
    ) {
        let offset = self.offset.get();

        writer.memory.get()[offset..offset + N]
            .iter()
            .zip(values.iter())
            .for_each(|(cell, &byte)| cell.set(byte));

        self.offset.set(offset + N);
    }

    // specialization of write_array for u8
    pub fn write_u8<A: Allocatable>(&self, writer: &MemoryWriter<'_, A>, value: u8) {
        let offset = self.offset.get();

        writer.memory.get()[offset].set(value);

        self.offset.set(offset + 1);
    }

    // specialization of write_array for u32
    pub fn write_u32<A: Allocatable>(&self, writer: &MemoryWriter<'_, A>, value: u32) {
        let offset = self.offset.get();

        let value = value.to_le_bytes();
        let memory = writer.memory.get();

        memory[offset].set(value[0]);
        memory[offset + 1].set(value[1]);
        memory[offset + 2].set(value[2]);
        memory[offset + 3].set(value[3]);

        self.offset.set(offset + 4);
    }

    #[allow(dead_code)]
    pub fn write_bytes<A: Allocatable>(&self, writer: &MemoryWriter<'_, A>, bytes: &[u8]) {
        let offset = self.offset.get();

        let memory = writer.memory.get();
        memory[offset..offset + bytes.len()]
            .iter()
            .zip(bytes)
            .for_each(|(cell, &byte)| cell.set(byte));

        self.offset.set(offset + bytes.len());
    }
}
