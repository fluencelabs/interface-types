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

use thiserror::Error as ThisError;

pub const DEFAULT_MEMORY_INDEX: usize = 0;

pub trait Allocatable {
    fn allocate(&self, size: u32, type_tag: u32) -> Result<u32, AllocatableError>;
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
    #[error("call to allocated was failed")]
    AllocateCallFailed,

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
