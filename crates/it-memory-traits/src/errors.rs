use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum MemoryAccessError {
    #[error("Out-of-bound Wasm memory access: offset {offset}, size {size}, while memory_size {memory_size}")]
    OutOfBounds {
        offset: usize,
        size: usize,
        memory_size: usize,
    },
}
