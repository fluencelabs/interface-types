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

use futures::future::BoxFuture;
use it_memory_traits::MemoryView;
use thiserror::Error as ThisError;

pub const DEFAULT_MEMORY_INDEX: usize = 0;

pub trait Allocatable<MV: MemoryView<Store>, Store: it_memory_traits::Store>: Send {
    fn allocate<'this, 'ctx1: 'this, 'ctx2: 'this>(
        &'this mut self,
        store: &'ctx1 mut <Store as it_memory_traits::Store>::ActualStore<'ctx2>,
        size: u32,
        type_tag: u32,
    ) -> BoxFuture<'this, Result<(u32, MV), AllocatableError>>;
}

#[derive(Debug, ThisError)]
pub enum AllocatableError {
    /// The memory doesn't exist.
    #[error("memory `{memory_index}` does not exist")]
    MemoryIsMissing {
        /// The memory index.
        memory_index: usize,
    },

    /// The local or import function doesn't exist.
    #[error("the allocate function with index `{function_index}` doesn't exist in Wasm module")]
    AllocateFuncIsMissing {
        /// The local or import function index.
        function_index: u32,
    },

    /// Failed to call a allocate function.
    #[error(r#"call to allocate function was failed: {reason}"#)]
    AllocateCallFailed {
        /// error returned by the allocate function
        #[source]
        reason: anyhow::Error,
    },

    /// Allocate input types doesn't match with needed.
    #[error(
        "allocate func doesn't receive two i32 values,\
             probably a Wasm module's built with unsupported sdk version"
    )]
    AllocateFuncIncompatibleSignature,

    /// Allocate output types doesn't match with needed.
    #[error(
        "allocate func doesn't return a one value of I32 type,\
             probably a Wasm module's built with unsupported sdk version"
    )]
    AllocateFuncIncompatibleOutput,

    // TODO: make it generic in future.
    /// User defined error.
    #[error("{0}")]
    UserDefinedError(String),
}
