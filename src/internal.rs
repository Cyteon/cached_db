use bson::{doc, Bson, Document};
use std::fs::{create_dir, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn ensure_folder(path: &String) {
    if Path::new(path).exists() {
        if !Path::new(path).is_dir() {
            println!("{} is not a directory", path);
            return;
        }
    } else {
        create_dir(path).unwrap();
    }
}
