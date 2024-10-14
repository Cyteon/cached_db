pub mod internal;
pub mod util;

use bson::Document;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;

pub struct Database {
    pub path: String,
    pub cache: Arc<Mutex<Option<HashMap<String, bson::Document>>>>,
    pub cache_duration: Duration,
}

impl Database {
    pub fn new(path: String, cache_duration: Duration) -> Database {
        let db = Database {
            path: path.to_string(),
            cache: Arc::new(Mutex::new(None)),
            cache_duration,
        };

        let cache = db.cache.clone();

        tokio::spawn(async move {
            loop {
                time::sleep(cache_duration).await;
                let mut cache_lock = cache.lock().unwrap();
                *cache_lock = Some(HashMap::new());
            }
        });

        db
    }

    pub fn ensure_folder(&self) {
        internal::ensure_folder(&self.path);
    }

    pub fn get_collection(
        &self,
        col: String,
    ) -> Result<bson::Document, Box<dyn std::error::Error>> {
        util::collection::get(&self.path, col)
    }

    pub fn drop_collection(&self, col: String) {
        util::collection::drop(&self.path, col);
    }

    pub fn insert_one(
        &self,
        col: String,
        obj: bson::Bson,
    ) -> Result<(), Box<dyn std::error::Error>> {
        util::insert::insert_one(&self.path, col, obj)
    }

    pub fn insert_many(
        &self,
        col: String,
        objs: Vec<bson::Bson>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        util::insert::insert_many(&self.path, col, objs)
    }

    pub fn get_one(
        &mut self,
        col: String,
        filter: bson::Document,
    ) -> Result<Option<bson::Document>, Box<dyn std::error::Error>> {
        util::get::get_one(self, col, filter)
    }

    pub fn get_one_no_cache(
        &mut self,
        col: String,
        filter: bson::Document,
    ) -> Result<Option<bson::Document>, Box<dyn std::error::Error>> {
        util::get::get_one_no_cache(self, col, filter)
    }

    pub fn update_one(
        &self,
        col: String,
        filter: bson::Document,
        update: bson::Document,
    ) -> Result<Option<Document>, Box<dyn std::error::Error>> {
        util::update::update_one(&self.path, col, filter, update)
    }
}
