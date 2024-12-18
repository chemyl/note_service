use crate::models::note::NewNote;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize)]
pub struct CreatedNote {
    pub id: i32,
    pub title: String,
    pub content: String,
}

pub async fn add_note(pool: web::Data<SqlitePool>, new_note: web::Json<NewNote>) -> impl Responder {
    match sqlx::query!(
        "INSERT INTO notes (title, content) VALUES (?, ?) RETURNING id",
        new_note.title,
        new_note.content
    )
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(record) => {
            HttpResponse::Created().json(CreatedNote {
                id: record.id as i32,
                title: new_note.title.clone(),
                content: new_note.content.clone(),
            })
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
