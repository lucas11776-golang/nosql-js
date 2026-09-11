use napi_derive::napi;
use nosql::{PageId, SlotId, Value};
use serde_derive::{Deserialize, Serialize};

// #[napi(object)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecordId {
    pub page_id: PageId,
    pub slot_id: SlotId,
}

impl From<nosql::RecordId> for RecordId {
    fn from(value: nosql::RecordId) -> Self {
        todo!()
    }
}

// #[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertResult {
    pub inserted_id: Value,
}

impl From<nosql::InsertResult> for InsertResult {
    fn from(value: nosql::InsertResult) -> Self {
        todo!()
    }
}

// #[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub matched_count: u64,
    pub modified_count: usize,
}


impl From<nosql::UpdateResult> for UpdateResult {
    fn from(value: nosql::UpdateResult) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResult {
    pub deleted_count: usize,
}

impl From<nosql::DeleteResult> for DeleteResult {
    fn from(value: nosql::DeleteResult) -> Self {
        todo!()
    }
}
