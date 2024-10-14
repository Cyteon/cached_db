use std::io::Write;

use bson::{Bson, Document};

use crate::internal;

use super::{cache::remove_cache, collection};

pub fn insert_one(path: &String, col: String, obj: Bson) -> Result<(), Box<dyn std::error::Error>> {
    internal::ensure_folder(path);

    let data = collection::get(path, col.clone());

    let mut doc = match data {
        Ok(doc) => doc,
        Err(_) => Document::new(),
    };

    let mut new_data = doc
        .get(&col)
        .unwrap_or(&Bson::Array(Vec::new()))
        .as_array()
        .unwrap()
        .to_vec();

    new_data.push(obj);

    doc.insert(&col, new_data);

    let mut file = std::fs::File::create(format!("{}/{}.bson", path, &col))?;
    file.write_all(bson::to_vec(&doc)?.as_slice())?;

    Ok(())
}

pub fn insert_many(
    path: &String,
    col: String,
    objs: Vec<Bson>,
) -> Result<(), Box<dyn std::error::Error>> {
    internal::ensure_folder(path);

    let data = collection::get(path, col.clone());

    let mut doc = match data {
        Ok(doc) => doc,
        Err(_) => Document::new(),
    };

    let mut new_data = doc
        .get(&col)
        .unwrap_or(&Bson::Array(Vec::new()))
        .as_array()
        .unwrap()
        .to_vec();

    for obj in objs {
        new_data.push(obj);
    }

    doc.insert(&col, new_data);

    let mut file = std::fs::File::create(format!("{}/{}.bson", path, &col))?;
    file.write_all(bson::to_vec(&doc)?.as_slice())?;

    Ok(())
}
