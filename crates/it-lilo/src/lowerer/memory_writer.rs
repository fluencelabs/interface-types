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
use crate::utils::type_tag_form_itype;

use it_memory_traits::MemoryView;
use std::cell::{Cell, RefCell};

pub struct MemoryWriter<'i, R: Allocatable<MV>, MV: MemoryView> {
    heap_manager: &'i R,
    view: RefCell<MV>,
}

impl<'i, A: Allocatable<MV>, MV: MemoryView> MemoryWriter<'i, A, MV> {
    pub fn new(view: MV, heap_manager: &'i A) -> LoResult<Self> {
        let writer = Self {
            heap_manager,
            view: RefCell::new(view),
        };
        Ok(writer)
    }

    pub fn write_bytes(&self, bytes: &[u8]) -> LoResult<u32> {
        let byte_type_tag = type_tag_form_itype(&crate::IType::U8);
        let seq_writer = self.sequential_writer(bytes.len() as u32, byte_type_tag)?;
        seq_writer.write_bytes(&self, bytes);

        Ok(seq_writer.start_offset())
    }

    pub fn sequential_writer(&self, size: u32, type_tag: u32) -> LoResult<SequentialWriter> {
        let (offset, view) = self.heap_manager.allocate(size, type_tag)?;
        self.view.replace(view);
        let seq_writer = SequentialWriter::new(offset);
        Ok(seq_writer)
    }
}

pub struct SequentialWriter {
    start_offset: u32,
    offset: Cell<u32>,
}

impl SequentialWriter {
    pub(self) fn new(offset: u32) -> Self {
        Self {
            offset: Cell::new(offset),
            start_offset: offset,
        }
    }

    pub fn start_offset(&self) -> u32 {
        self.start_offset
    }

    pub fn write_array<MV: MemoryView, A: Allocatable<MV>, const N: usize>(
        &self,
        writer: &MemoryWriter<'_, A, MV>,
        values: [u8; N],
    ) {
        let offset = self.offset.get();

        writer.view.borrow().write_bytes(offset, values);

        self.offset.set(offset + N as u32);
    }

    // specialization of write_array for u8
    pub fn write_u8<MV: MemoryView, A: Allocatable<MV>>(
        &self,
        writer: &MemoryWriter<'_, A, MV>,
        value: u8,
    ) {
        let offset = self.offset.get();

        writer.view.borrow().write_byte(offset, value);

        self.offset.set(offset + 1);
    }

    // specialization of write_array for u32
    pub fn write_u32<MV: MemoryView, A: Allocatable<MV>>(
        &self,
        writer: &MemoryWriter<'_, A, MV>,
        value: u32,
    ) {
        let offset = self.offset.get();

        let value = value.to_le_bytes();
        writer.view.borrow().write_bytes(offset, value);

        self.offset.set(offset + 4);
    }

    #[allow(dead_code)]
    pub fn write_bytes<MV: MemoryView, A: Allocatable<MV>>(
        &self,
        writer: &MemoryWriter<'_, A, MV>,
        bytes: &[u8],
    ) {
        let offset = self.offset.get();

        writer.view.borrow().write_slice(offset, bytes);

        self.offset.set(offset + bytes.len() as u32);
    }
}
