#![deny(clippy::all)]

use std::sync::Arc;

use napi_derive::napi;
use napi::Result;

use crate::collection::Collection;

pub mod collection;
pub mod types;

#[napi]
pub struct Database {
    inner: Arc<nosql::Database>
}

#[napi]
impl Database {
    #[napi]
    pub async fn open(path: String) -> Result<Database> {
        Ok(Self {
            inner: nosql::Database::open(&path).await.unwrap()
        })
    }

    #[napi]
    pub async fn collection(&self, name: String) -> Collection {
        Collection::new(self.inner.collection(&name).await)
    }

    #[napi]
    pub async fn create(&self, collection: String) -> Result<()> {
        self
            .inner
            .create(&collection)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
    }
    
    #[napi]
    pub async fn rename(&self, collection_old: String, collection_new: String) -> Result<()> {
        self
            .inner
            .rename(&collection_old, &collection_new)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
    }

    #[napi]
    pub async fn delete(&self, name: String) -> Result<()> {
        self
            .inner
            .delete(&name)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
    }

    #[napi]
    pub async fn has(&self, collection: String) -> bool {
        self
            .inner
            .has(&collection)
            .await
    }

    #[napi]
    pub async fn list(&self) -> Vec<String> {
        self
            .inner
            .list()
            .await
    }

    #[napi]
    pub async fn close(&self) -> Result<()> {
        self
            .inner
            .close()
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:#}", e)))
            
    }
}