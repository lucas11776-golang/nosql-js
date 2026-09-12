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
    pub async fn insert_one(&self, doc: Value) -> Result<InsertResult> {
        self
            .inner
            .insert_one(doc)
            .await
            .map(|v| InsertResult::from(v))
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
    }


    #[napi]
    pub async fn insert_many(&self, docs: Vec<Value>) -> Result<Vec<InsertResult>> {
        self
            .inner
            .insert_many(docs)
            .await
            .map(|v| {
                v
                    .iter()
                    .map(|v| InsertResult::from(v.clone())) // TODO: performance million records.
                    .collect::<Vec<InsertResult>>()
            })
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
    }

    #[napi(
        ts_generic_types = "T",
        ts_return_type = "Promise<T>"
    )]
    pub async fn find_one(&self, filter: Value) -> Result<Option<Value>> {
        self
            .inner
            .find_one(filter)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
    }

    #[napi(
        ts_generic_types = "T",
        ts_return_type = "Promise<T>"
    )]
    pub async fn find(&self, filter: Value) -> Result<Vec<Value>> {
        self
            .inner
            .find(filter)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
    }

    #[napi]
    pub async fn update(&self, filter: Value, update: Value) -> Result<UpdateResult> {
        self
            .inner
            .update(filter, update)
            .await
            .map(|v| UpdateResult::from(v))
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))

    }

    #[napi]
    pub async fn delete(&self, filter: Value) -> Result<DeleteResult> {
        self
            .inner
            .delete(filter)
            .await
            .map(|v| DeleteResult::from(v))
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
    }
}