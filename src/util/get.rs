use std::io::Write;

use bson::{Bson, Document};

use crate::internal;

use super::{cache::set_cache, collection};

pub fn get_one_no_cache(
    db: &mut crate::Database,
    col: String,
    filter: Document,
) -> Result<Option<Document>, Box<dyn std::error::Error + Send + Sync>> {
    internal::ensure_folder(&db.path);

    let data = collection::get(&db.path, col.clone());

    let doc = match data {
        Ok(doc) => doc,
        Err(_) => Document::new(),
    };

    let docs = match doc.get(&col) {
        Some(Bson::Array(docs)) => docs,
        _ => return Err("Collection not found".into()),
    };

    for d in docs {
        if let Bson::Document(d) = d {
            if document_matches_filter(d, &filter) {
                return Ok(Some(d.clone()));
            }
        }
    }

    Err("Not found".into())
}

pub fn get_one(
    db: &mut crate::Database,
    col: String,
    filter: Document,
) -> Result<Option<Document>, Box<dyn std::error::Error + Send + Sync>> {
    internal::ensure_folder(&db.path);

    if let Some(cache) = &*db.cache.lock().unwrap() {
        if let Some(doc) = cache.get(&format!("{}-{}", col, filter)) {
            return Ok(Some(doc.clone()));
        }
    }

    let data = collection::get(&db.path, col.clone());

    let doc = match data {
        Ok(doc) => doc,
        Err(_) => Document::new(),
    };

    let docs = match doc.get(&col) {
        Some(Bson::Array(docs)) => docs,
        _ => return Err("Collection not found".into()),
    };

    for d in docs {
        if let Bson::Document(d) = d {
            if document_matches_filter(d, &filter) {
                set_cache(db, col, filter, d);
                return Ok(Some(d.clone()));
            }
        }
    }

    Err("Not found".into())
}

pub fn get_many(
    db: &mut crate::Database,
    col: String,
    filter: Document,
) -> Result<Vec<Document>, Box<dyn std::error::Error + Send + Sync>> {
    internal::ensure_folder(&db.path);

    let data = collection::get(&db.path, col.clone());

    let doc = match data {
        Ok(doc) => doc,
        Err(_) => Document::new(),
    };

    let docs = match doc.get(&col) {
        Some(Bson::Array(docs)) => docs,
        _ => return Err("Collection not found".into()),
    };

    let mut results = Vec::new();

    for d in docs {
        if let Bson::Document(d) = d {
            if document_matches_filter(d, &filter) {
                results.push(d.clone());
            }
        }
    }

    Ok(results)
}

fn document_matches_filter(doc: &Document, filter: &Document) -> bool {
    for (key, value) in filter {
        if doc.get(key) != Some(value) {
            return false;
        }
    }

    true
}
