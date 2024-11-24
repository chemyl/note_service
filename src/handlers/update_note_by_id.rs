use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct UpdateNoteContent {
    pub content: String,
}

pub async fn update_note_by_id(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
    json: web::Json<UpdateNoteContent>) -> HttpResponse {
    let id = path.into_inner();
    let new_content = &json.content;
    let rows_affected = sqlx::query!("UPDATE notes SET content = ? WHERE id = ?",new_content,id)
        .execute(pool.get_ref())
        .await;

    match rows_affected {
        Ok(result) if result.rows_affected() > 0 => HttpResponse::Ok().finish(), // Successfully updated
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}