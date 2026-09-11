use napi::Result;
use napi_derive::napi;
use nosql::Value;

use crate::types::{DeleteResult, InsertResult, UpdateResult};

#[napi]
pub struct Collection {
    inner: nosql::engine::collection::Collection,
}

#[napi]
impl Collection {
    pub fn new(collection: nosql::engine::collection::Collection) -> Self {
        Self {
            inner: collection
        }
    }

    #[napi]
    pub async fn insert_one(&self, mut doc: Value) -> Result<InsertResult> {
        self
            .inner
            .insert_one(doc)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
            .map(|v| InsertResult::from(v))
    }


    #[napi]
    pub async fn insert_many(&self, docs: Vec<Value>) -> Result<Vec<InsertResult>> {
        todo!()
    }

    #[napi]
    pub async fn find_one(&self, filter: Value) -> Result<Option<Value>> {
        todo!()
    }

    #[napi]
    pub async fn update(&self, filter: Value, update: Value) -> Result<UpdateResult> {
        todo!()
    }

    pub async fn delete(&self, filter: Value) -> Result<DeleteResult> {
        todo!()
    }
}