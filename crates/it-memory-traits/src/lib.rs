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

pub trait Store {
    type ActualStore<'c>;
}

pub trait MemoryReadable<Store: self::Store> {
    /// This function will panic if the `offset` is out of bounds.
    /// It is caller's responsibility to check if the offset is in bounds
    /// using `MemoryView::check_bounds` function
    fn read_byte(&self, store: &mut <Store as self::Store>::ActualStore<'_>, offset: u32) -> u8;

    /// This function will panic if `[offset..offset + COUNT]` is out of bounds.
    /// It is caller's responsibility to check if the offset is in bounds
    /// using `MemoryView::check_bounds` function.
    fn read_array<const COUNT: usize>(
        &self,
        store: &mut <Store as self::Store>::ActualStore<'_>,
        offset: u32,
    ) -> [u8; COUNT];

    /// This function will panic if `[offset..offset + size]` is out of bounds.
    /// It is caller's responsibility to check if the offset is in bounds
    /// using `MemoryView::check_bounds` function.
    fn read_vec(
        &self,
        store: &mut <Store as self::Store>::ActualStore<'_>,
        offset: u32,
        size: u32,
    ) -> Vec<u8>;
}

pub trait MemoryWritable<Store: self::Store> {
    /// This function will panic if `offset` is out of bounds.
    /// It is caller's responsibility to check if the offset is in bounds
    /// using `MemoryView::check_bounds` function.
    fn write_byte(
        &self,
        store: &mut <Store as self::Store>::ActualStore<'_>,
        offset: u32,
        value: u8,
    );

    /// This function will panic if `[offset..offset + bytes.len()]`.is out of bounds.
    /// It is caller's responsibility to check if the offset is in bounds
    /// using `MemoryView::check_bounds` function.
    fn write_bytes(
        &self,
        store: &mut <Store as self::Store>::ActualStore<'_>,
        offset: u32,
        bytes: &[u8],
    );
}

pub trait MemoryView<Store: self::Store>: MemoryWritable<Store> + MemoryReadable<Store> {
    /// For optimization purposes, user must check bounds first, then try read-write to memory
    /// `MemoryWritable` and `MemoryReadable` functions will panic in case of out of bounds access`
    fn check_bounds(
        &self,
        store: &mut <Store as self::Store>::ActualStore<'_>,
        offset: u32,
        size: u32,
    ) -> Result<(), MemoryAccessError>;
}

pub trait Memory<View, Store: self::Store>
where
    View: MemoryView<Store>,
{
    fn view(&self) -> View;
}
