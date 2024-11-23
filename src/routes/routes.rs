use crate::handlers;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notes")
            .route("", web::get().to(handlers::get_notes))
            .route("", web::post().to(handlers::add_note))
            .route("/{id}", web::get().to(handlers::get_note_by_id)),
    );
}