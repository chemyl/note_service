use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

pub async fn delete_note_by_id(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM notes WHERE id = ?", id)
        .execute(pool.get_ref())
        .await;
    match rows_affected {
        Ok(result) if result.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}