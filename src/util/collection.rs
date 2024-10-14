use bson::Document;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn get(
    path: &String,
    col: String,
) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
    let mut file = File::open(format!("{}/{}.bson", path, col))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let doc = bson::from_slice(&buffer)?;
    Ok(doc)
}

pub fn drop(path: &String, col: String) {
    if Path::new(&format!("{}", path)).exists() {
        if Path::new(&format!("{}", path)).is_dir() {
            if Path::new(&format!("{}/{}.bson", path, col)).exists() {
                match std::fs::remove_file(format!("{}/{}.bson", path, col)) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
}
