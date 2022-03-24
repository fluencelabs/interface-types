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

#[macro_export]
macro_rules! value_der {
    ($self:expr, $offset:expr, @seq_start $($ids:tt),* @seq_end) => {
        [$($self.memory.get($offset + $ids)),+]
    };

    ($self:expr, $offset:expr, 1) => {
        crate::value_der!($self, $offset, @seq_start 0 @seq_end);
    };

    ($self:expr, $offset:expr, 2) => {
        crate::value_der!($self, $offset, @seq_start 0, 1 @seq_end);
    };

    ($self:expr, $offset:expr, 4) => {
        crate::value_der!($self, $offset, @seq_start 0, 1, 2, 3 @seq_end);
    };

    ($self:expr, $offset:expr, 8) => {
        crate::value_der!($self, $offset, @seq_start 0, 1, 2, 3, 4, 5, 6, 7 @seq_end);
    };

    ($self:expr, $offset:expr, 16) => {
        crate::value_der!($self, $offset, @seq_start 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 @seq_end);
    };
}

#[macro_export]
macro_rules! read_ty {
    ($func_name:ident, $ty:ty, 1) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 1));

            self.offset.set(offset + 1);
            result
        }
    };

    ($func_name:ident, $ty:ty, 2) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 2));

            self.offset.set(offset + 2);
            result
        }
    };

    ($func_name:ident, $ty:ty, 4) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 4));

            self.offset.set(offset + 4);
            result
        }
    };

    ($func_name:ident, $ty:ty, 8) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 8));

            self.offset.set(offset + 8);
            result
        }
    };

    ($func_name:ident, $ty:ty, 16) => {
        fn $func_name(&self) -> $ty {
            let offset = self.offset.get();
            let result = <$ty>::from_le_bytes(crate::value_der!(self, offset, 16));

            self.offset.set(offset + 16);
            result
        }
    };
}

#[macro_export]
macro_rules! read_ty_decl {
    ($func_name:ident, $ty:ty, 1) => {
        fn $func_name(&self) -> $ty;
    };

    ($func_name:ident, $ty:ty, 2) => {
        fn $func_name(&self) -> $ty;
    };

    ($func_name:ident, $ty:ty, 4) => {
        fn $func_name(&self) -> $ty;
    };

    ($func_name:ident, $ty:ty, 8) => {
        fn $func_name(&self) -> $ty;
    };

    ($func_name:ident, $ty:ty, 16) => {
        fn $func_name(&self) -> $ty;
    };
}

#[macro_export]
macro_rules! read_array_ty {
    ($func_name:ident, $ty:ident, $ity:ident) => {
        pub fn $func_name(
            &self,
            offset: u32,
            elements_count: u32,
        ) -> super::LiResult<Vec<crate::IValue>> {
            let reader = self
                .sequential_reader(offset, (std::mem::size_of::<$ty>() as u32) * elements_count)?;
            let mut result = Vec::with_capacity(elements_count as usize);

            for _ in 0..elements_count {
                let value = paste::paste! { reader.[<read_ $ty>]()};
                result.push(IValue::$ity(value));
            }

            Ok(result)
        }
    };
}
