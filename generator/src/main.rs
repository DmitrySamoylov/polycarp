use actix_web::{post, web::Json, App, HttpServer, Responder};
use problem::{add_problem, Problem};

mod problem;

const SERVER_ADDR: &str = "127.0.0.1:27121";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(SERVER_ADDR)?
        .run()
        .await
}

#[post("/")]
async fn index(Json(problem): Json<Problem>) -> impl Responder {
    if let Err(e) = add_problem(problem).await {
        eprintln!("Failed to add problem: {:?}", e)
    }

    "ok"
}
