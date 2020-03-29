use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    println!("Hello, client!");

    HttpResponse::Ok().body("Hello, client!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    tokio::spawn(async move {
        loop {
            println!("Hello, server!");
            tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
        }
    });
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
