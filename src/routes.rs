use actix_web::web;
use crate::handlers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/notes") // new "scope" /notes
                     .route("", web::get().to(handlers::get_notes))
                     .route("", web::post().to(handlers::add_note)),
    );
}
