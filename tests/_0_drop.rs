#[tokio::test]
async fn drop_col() {
    let db = cached_db::Database::new("db".to_string(), std::time::Duration::from_secs(60));
    db.drop_collection("test".to_string());
}
