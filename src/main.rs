mod router;

use std::env;
use env_logger::init;
use actix_web::{ App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    init();

    let address = format!("{}:{}",
                env::var("APP_HOST").expect("Setting Host dulu"),
                env::var("APP_PORT").expect("Setting Port dulu"));

    let server = HttpServer::new( || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .configure(router::routers::config)})
        .bind(address).unwrap();

        println!("Server Berjalan ");
        server.run().await
}