use std::time::Duration;

use bson::{bson, doc, Bson};
use cached_db::Database;

#[tokio::test]
async fn get_test() {
    let mut db = Database::new("db".to_string(), Duration::from_secs(60));
    db.drop_collection("test".to_string());

    db.insert_one(
        "test".to_string(),
        Bson::Document(doc! {
            "string": "Hello, World!",
            "int": 42,
            "float": 3.14,
            "bool": true,
        }),
    )
    .unwrap();

    let time_start = std::time::Instant::now();

    let result = db.get_one("test".to_string(), doc! {"string": "Hello, World!"});

    match result {
        Ok(doc) => {
            assert_eq!(doc.unwrap().get("float").unwrap().as_f64().unwrap(), 3.14);
        }
        Err(_) => assert!(false),
    }

    println!("get_test: {:?}", time_start.elapsed());
}

#[tokio::test]
async fn get_1000() {
    let mut db = Database::new("db".to_string(), Duration::from_secs(60));
    db.drop_collection("test".to_string());

    // We add 1000 documents to the database first
    for i in 0..1000 {
        db.insert_one(
            "test".to_string(),
            Bson::Document(doc! {
                "string": format!("Hello, World! {}", i),
                "int": i,
                "float": 3.14,
                "bool": true,
            }),
        )
        .unwrap();
    }

    let time_start = std::time::Instant::now();

    for i in 0..1000 {
        let result = db.get_one(
            "test".to_string(),
            doc! {"string": format!("Hello, World! {}", i)},
        );

        match result {
            Ok(doc) => {
                assert_eq!(doc.unwrap().get("int").unwrap().as_i32().unwrap(), i);
            }
            Err(_) => assert!(false),
        }
    }

    println!("get_1000: {:?}", time_start.elapsed());
}

#[tokio::test]
async fn get_1000_cache() {
    let mut db = Database::new("db".to_string(), Duration::from_secs(60));

    let time_start = std::time::Instant::now();

    for _ in 0..1000 {
        let result = db.get_one_cached("test".to_string(), doc! {"bool": true}); // Same filter = cache hit

        match result {
            Ok(doc) => {
                assert_eq!(doc.unwrap().get("bool").unwrap().as_bool().unwrap(), true);
            }
            Err(_) => assert!(false),
        }
    }

    println!("get_1000_cache: {:?}", time_start.elapsed());
}
