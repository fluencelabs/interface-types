use it_lilo_utils::error::MemoryAccessError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub(crate) enum LiLoRecordError {
    /// This error occurred from out-of-bound memory access.
    #[error("{0}")]
    MemoryAccessError(#[from] MemoryAccessError),

    /// An error related to not found record in module record types.
    #[error("Record with type id {0} not found")]
    RecordTypeNotFound(u64),

    /// This error occurred when a record is created from empty values array.
    #[error("Record with name '{0}' can't be empty")]
    EmptyRecord(String),
}
