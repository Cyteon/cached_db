use std::io::Write;

use bson::{Bson, Document};

use crate::internal;

use super::collection;

pub fn update_one(
    path: &String,
    col: String,
    filter: Document,
    update: Document,
) -> Result<Option<Document>, Box<dyn std::error::Error>> {
    internal::ensure_folder(path);

    let data = collection::get(path, col.clone());

    let mut doc = match data {
        Ok(doc) => doc,
        Err(_) => Document::new(),
    };

    let docs = match doc.get(&col) {
        Some(Bson::Array(docs)) => docs.clone(),
        _ => return Err("Collection not found".into()),
    };

    for (index, d) in docs.iter().enumerate() {
        if let Bson::Document(d) = d {
            if document_matches_filter(d, &filter) {
                let mut new_data = d.clone();

                for (key, value) in &update {
                    new_data.insert(key, value);
                }

                let mut new_docs = docs.clone();
                new_docs[index] = Bson::Document(new_data.clone());

                doc.insert(&col, new_docs);

                let mut file = std::fs::File::create(format!("{}/{}.bson", path, &col))?;
                file.write_all(bson::to_vec(&doc)?.as_slice())?;

                return Ok(Some(new_data));
            }
        }
    }

    Err("Not found".into())
}

fn document_matches_filter(doc: &Document, filter: &Document) -> bool {
    for (key, value) in filter {
        if doc.get(key) != Some(value) {
            return false;
        }
    }

    true
}
