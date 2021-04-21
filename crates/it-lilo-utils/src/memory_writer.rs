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

use crate::error::MemoryAccessError;
use crate::MResult;

use std::cell::Cell;

pub struct MemoryWriter<'m> {
    memory: &'m [Cell<u8>],
}

/// Writes values of basic types sequentially to the provided writer.
/// It don't check memory limits for the optimization purposes,
/// so it could be created only by the MemoryReader::sequential_reader method.
pub struct SequentialWriter<'w, 'm> {
    writer: &'w MemoryWriter<'m>,
    offset: Cell<usize>,
}

impl<'m> MemoryWriter<'m> {
    pub fn new(memory: &'m [Cell<u8>]) -> Self {
        Self { memory }
    }

    pub fn write_array<const N: usize>(&self, offset: usize, values: [u8; N]) -> MResult<()> {
        self.check_access(offset, values.len())?;

        self.memory[offset..offset + N]
            .iter()
            .zip(values.iter())
            .for_each(|(cell, &byte)| cell.set(byte));

        Ok(())
    }

    // specialization of write_array for u8
    pub fn write_u8(&self, offset: usize, value: u8) -> MResult<()> {
        self.check_access(offset, 1)?;
        self.memory[offset].set(value);

        Ok(())
    }

    // specialization of write_array for u32
    pub fn write_u32(&self, offset: usize, value: u32) -> MResult<()> {
        self.check_access(offset, 4)?;

        let value = value.to_le_bytes();
        self.memory[offset].set(value[0]);
        self.memory[offset + 1].set(value[1]);
        self.memory[offset + 2].set(value[2]);
        self.memory[offset + 3].set(value[3]);

        Ok(())
    }

    pub fn write_bytes(&self, offset: usize, bytes: &[u8]) -> MResult<()> {
        let writer = self.sequential_writer(offset, bytes.len())?;
        writer.write_bytes(bytes);

        Ok(())
    }

    pub fn sequential_writer(
        &self,
        offset: usize,
        size: usize,
    ) -> MResult<SequentialWriter<'_, '_>> {
        self.check_access(offset, size)?;

        Ok(SequentialWriter::new(&self, offset))
    }

    pub fn check_access(&self, offset: usize, size: usize) -> MResult<()> {
        let right = offset + size;

        // the first condition is a check for overflow
        if right < offset || right >= self.memory.len() {
            return Err(MemoryAccessError::InvalidAccess {
                offset,
                size,
                memory_size: self.memory.len(),
            });
        }

        Ok(())
    }
}

impl<'w, 'm> SequentialWriter<'w, 'm> {
    pub(super) fn new(writer: &'w MemoryWriter<'m>, offset: usize) -> Self {
        let offset = Cell::new(offset);

        Self { writer, offset }
    }

    pub fn write_array<const N: usize>(&self, values: [u8; N]) {
        let offset = self.offset.get();

        self.writer.memory[offset..offset + N]
            .iter()
            .zip(values.iter())
            .for_each(|(cell, &byte)| cell.set(byte));

        self.offset.set(offset + N);
    }

    // specialization of write_array for u8
    pub fn write_u8(&self, value: u8) {
        let offset = self.offset.get();

        self.writer.memory[offset].set(value);

        self.offset.set(offset + 1);
    }

    // specialization of write_array for u32
    pub fn write_u32(&self, value: u32) {
        let offset = self.offset.get();

        let value = value.to_le_bytes();
        self.writer.memory[offset].set(value[0]);
        self.writer.memory[offset + 1].set(value[1]);
        self.writer.memory[offset + 2].set(value[2]);
        self.writer.memory[offset + 3].set(value[3]);

        self.offset.set(offset + 4);
    }

    #[allow(dead_code)]
    pub fn write_bytes(&self, bytes: &[u8]) {
        let offset = self.offset.get();

        self.writer.memory[offset..offset + bytes.len()]
            .iter()
            .zip(bytes)
            .for_each(|(cell, &byte)| cell.set(byte));

        self.offset.set(offset + bytes.len());
    }
}
