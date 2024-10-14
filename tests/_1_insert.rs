use bson::{bson, doc, Bson};
use cached_db::Database;
use std::time::Duration;

#[tokio::test]
async fn insert_test() {
    let time_start = std::time::Instant::now();

    let db = Database::new("db".to_string(), Duration::from_secs(60));

    let result = db.insert_one(
        "test".to_string(),
        Bson::Document(doc! {
            "string": "Hello, World!",
            "int": 42,
            "float": 3.14,
            "bool": true,
        }),
    );

    match result {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }

    println!("insert_test: {:?}", time_start.elapsed());
}

#[tokio::test]
async fn insert_1000() {
    let db = Database::new("db".to_string(), Duration::from_secs(60));

    let time_start = std::time::Instant::now();

    for i in 0..1000 {
        let result = db.insert_one(
            "test".to_string(),
            Bson::Document(doc! {
                "string": format!("Hello, World! {}", i),
                "int": i,
                "float": 3.14,
                "bool": true,
            }),
        );

        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    println!("insert_1000: {:?}", time_start.elapsed());
}
