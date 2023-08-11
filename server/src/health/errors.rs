use serde::{ser::SerializeStruct, Serialize};

use super::types::UnhealthyStatus;

pub struct ServiceUnhealthyError {
    pub status: UnhealthyStatus,
}

impl Serialize for ServiceUnhealthyError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut error = serializer.serialize_struct("error", 2)?;
        let _ = error.serialize_field("type", "ServiceUnhealthyError");
        let _ = error.serialize_field("status", &self.status);
        error.end()
    }
}
