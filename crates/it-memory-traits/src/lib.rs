/*
 * Copyright 2022 Fluence Labs Limited
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

mod errors;

pub use errors::MemoryAccessError;

pub trait MemoryReadable {
    fn read_byte(&self, offset: u32) -> u8;

    fn read_bytes<const COUNT: usize>(&self, offset: u32) -> [u8; COUNT];

    fn read_vec(&self, offset: u32, size: u32) -> Vec<u8>;
}

pub trait MemoryWritable {
    fn write_byte(&self, offset: u32, value: u8);

    fn write_bytes<const COUNT: usize>(&self, offset: u32, value: [u8; COUNT]);

    fn write_slice(&self, offset: u32, bytes: &[u8]);
}

pub trait MemoryView: MemoryWritable + MemoryReadable {
    // For optimization purposes, user must check bounds first, then try read-write to memory
    // MemoryWritable and MemoryReadable will panic in case of out of bounds access
    fn check_bounds(&self, offset: u32, size: u32) -> Result<(), MemoryAccessError>;
}

pub trait Memory<View>
where
    View: MemoryView,
{
    fn view(&self) -> View;
}
