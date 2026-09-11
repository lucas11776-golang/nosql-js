use napi_derive::napi;
use nosql::{PageId, SlotId, Value};
use serde_derive::{Deserialize, Serialize};

#[napi(object)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecordId {
    pub page_id: u32,
    pub slot_id: u16,
}

impl From<nosql::RecordId> for RecordId {
    fn from(value: nosql::RecordId) -> Self {
        Self {
            page_id: value.page_id,
            slot_id: value.slot_id,
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertResult {
    pub inserted_id: Value,
}

impl From<nosql::InsertResult> for InsertResult {
    fn from(value: nosql::InsertResult) -> Self {
        Self {
            inserted_id: value.inserted_id
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub matched_count: i64,
    pub modified_count: i64,
}


impl From<nosql::UpdateResult> for UpdateResult {
    fn from(value: nosql::UpdateResult) -> Self {
        todo!()
    }
}


#[napi]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResult {
    pub deleted_count: i64,
}

impl From<nosql::DeleteResult> for DeleteResult {
    fn from(value: nosql::DeleteResult) -> Self {
        todo!()
    }
}
