use crate::Database;
use std::collections::HashMap;

pub fn set_cache(db: &mut Database, col: String, filter: bson::Document, doc: &bson::Document) {
    let key = format!("{}-{}", col, filter.to_string());

    let mut cache_lock = db.cache.lock().unwrap();

    if cache_lock.is_none() {
        *cache_lock = Some(HashMap::new());
    }

    if let Some(cache) = cache_lock.as_mut() {
        cache.insert(key, doc.clone());
    }
}

pub fn remove_cache(db: &mut Database, col: String, filter: bson::Document) {
    let key = format!("{}-{}", col, filter.to_string());

    let mut cache_lock = db.cache.lock().unwrap();

    if let Some(cache) = cache_lock.as_mut() {
        cache.remove(&key);
    }
}
