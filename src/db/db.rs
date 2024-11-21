use sqlx::{sqlite::SqlitePool, Pool};

pub async fn init_db() -> Pool<sqlx::Sqlite> {
    let pool = SqlitePool::connect("sqlite:note_service.db").await.unwrap();     //TODO unwrap -> panic!
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL)"
    )
        .execute(&pool)
        .await
        .unwrap();          //TODO unwrap -> panic!
    pool        // SqlitePool
}