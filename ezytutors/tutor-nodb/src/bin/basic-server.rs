use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/kmj", web::get().to(kmj));
    cfg.route("/kmj1", web::get().to(kmj1));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("hello. EzyTutors is alive and kicking")
}

pub async fn kmj() -> impl Responder {
    HttpResponse::Ok().content_type("text/html; charset=utf-8")
                      .body("깐트롤+C<br>깐트롤+V ^_^/")
}

pub async fn kmj1() -> impl Responder {
    HttpResponse::Ok().json("깐트롤+C, 깐트롤+V ^_^/")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(general_routes);
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}