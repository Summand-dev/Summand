use actix_web::{get, App, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    format!("Summand api is running ...")
}

pub async fn run() -> std::io::Result<()> {
    println!("Starting api server at http://127.0.0.1:8373");
    HttpServer::new(|| App::new().service(health))
        .bind(("0.0.0.0", 8379))?
        .run()
        .await
}
