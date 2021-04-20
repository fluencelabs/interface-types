pub mod error;
pub mod memory_reader;
pub mod memory_writer;

pub use fluence_it_types::IValue;

pub type MResult<T> = std::result::Result<T, error::MemoryAccessError>;
