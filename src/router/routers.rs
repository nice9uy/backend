use actix_web::{web, HttpResponse};

/// Ini Halaman Home
pub fn config(cfg : &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(|| HttpResponse::Ok().body("Ini halaman Home")));
}