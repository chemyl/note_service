use crate::models::note::Note;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

pub async fn get_notes(pool: web::Data<SqlitePool>) -> HttpResponse {
    let notes: Vec<Note> = sqlx::query_as!(Note, "SELECT id, title, content FROM notes")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();   //or empty vec
    HttpResponse::Ok().json(notes)
}