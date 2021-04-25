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

use crate::IRecordType;
use thiserror::Error as ThisError;

pub trait RecordResolvable {
    fn resolve_record(&self, record_type_id: u64) -> Result<&IRecordType, RecordResolvableError>;
}

#[derive(Debug, ThisError)]
pub enum RecordResolvableError {
    /// Record for such type is wasn't found.
    #[error("Record with type id '{0}' not found")]
    RecordNotFound(u64),
}
