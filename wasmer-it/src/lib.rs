//! This crate contains an implementation of [WebAssembly Interface
//! Types][wit] (abbreviated WIT). It is composed of 5 parts:
//!
//! 1. [Types] and [Values]: To represent the WIT types and values
//!    representations,
//! 2. [AST]: To represent the WIT language as a tree
//!    (which is not really abstract). This is the central
//!    representation of the language.
//! 3. [Decoders](decoders): To read the [AST] from a particular data
//!    representation; for instance, [`decoders::binary::parse`] reads
//!    the [AST] from a binary.
//! 4. [Encoders](encoders): To write the [AST](ast) into a particular
//!    format; for instance, [`encoders::wat`] writes the [AST] into a
//!    string representing WIT with its textual format.
//! 5. [Interpreter](interpreter): WIT defines a concept called
//!    Adapters. An adapter contains a set of [instructions]. So, in
//!    more details, this module contains:
//!     * [A very light and generic stack
//!       implementation](interpreter::stack), exposing only the
//!       operations required by the interpreter,
//!     * [A stack-based interpreter](interpreter::Interpreter),
//!       defined by:
//!          * A compiler that transforms a set of instructions into a
//!            set of executable instructions,
//!          * A stack,
//!          * A runtime that holds the “invocation inputs” (arguments
//!            of the interpreter), the stack, and the WebAssembly
//!            instance (which holds the exports, the imports, the
//!            memories, the tables etc.),
//!     * [An hypothetic WebAssembly runtime](interpreter::wasm),
//!       represented as a set of enums, types, and traits —basically
//!       this is the part a runtime should take a look to use the
//!       `wasmer-interface-types` crate—.
//!
//! [wit]: https://github.com/WebAssembly/interface-types
//! [Types]: types
//! [Values]: values
//! [AST]: ast
//! [instructions]: interpreter::Instruction

#![deny(
    dead_code,
    rustdoc::broken_intra_doc_links,
    missing_docs,
    nonstandard_style,
    unreachable_patterns,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unused_variables
)]
// #![forbid(unsafe_code)]
#![doc(html_favicon_url = "https://wasmer.io/static/icons/favicon.ico")]
#![doc(html_logo_url = "https://github.com/wasmerio.png")]

pub mod ast;
#[macro_use]
pub mod macros;
pub mod decoders;
pub mod encoders;
pub mod errors;
pub mod interpreter;
#[cfg(feature = "serde")]
mod serde;
mod values;

// re-exports
pub use fluence_it_types::ne_vec::NEVec;
pub use fluence_it_types::IRecordFieldType;
pub use fluence_it_types::IRecordType;
pub use fluence_it_types::IType;
pub use fluence_it_types::IValue;

pub use it_to_bytes::ToBytes;

pub use crate::interpreter::record_size;

#[cfg(feature = "serde")]
pub use crate::serde::de::from_interface_values;

#[cfg(feature = "serde")]
pub use crate::serde::ser::to_interface_value;
