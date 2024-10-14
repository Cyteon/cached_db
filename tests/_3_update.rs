use std::time::Duration;

use bson::{doc, Bson};
use cached_db::Database;

#[tokio::test]
async fn update_test() {
    let mut db = Database::new("db".to_string(), Duration::from_secs(60));

    let time_start = std::time::Instant::now();

    db.update_one(
        "test".to_string(),
        doc! {"string": "Hello, World!"},
        doc! {"float": 1.1, "bool": false},
    )
    .unwrap();

    println!("update_test: {:?}", time_start.elapsed());

    let result = db.get_one("test".to_string(), doc! {"float": 1.1});

    match result {
        Ok(doc) => {
            assert_eq!(doc.unwrap().get("bool").unwrap().as_bool().unwrap(), false);
        }
        Err(_) => assert!(false),
    }
}
