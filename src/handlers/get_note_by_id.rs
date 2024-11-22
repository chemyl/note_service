use crate::models::note::Note;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

pub async fn get_note_by_id(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id = path.into_inner(); // Получение значения i32
    let note: Option<Note> = sqlx::query_as!(Note, "SELECT id, title, content FROM notes WHERE id = ?", id)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    match note {
        Some(note) => HttpResponse::Ok().json(note),
        None => HttpResponse::NotFound().finish(),
    }
}